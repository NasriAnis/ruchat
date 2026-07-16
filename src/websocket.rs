use std::collections::HashMap;
use std::io;
use std::net::{TcpListener, TcpStream};
use std::sync::mpsc::{Sender, channel};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use rustls::{ServerConfig, ServerConnection, StreamOwned};

use tungstenite::accept_hdr;
use tungstenite::handshake::server::{Request, Response};
use tungstenite::protocol::{Message, WebSocket};
use tungstenite::protocol::{CloseFrame, frame::coding::CloseCode};

use serde::{Deserialize, Serialize};

use crate::database;
use crate::json;

#[derive(Serialize, Deserialize, Debug, Default)]
struct MessageRcv {
    message: String,
}

type Clients = Arc<Mutex<HashMap<String, Sender<Message>>>>;
type TlsStream = StreamOwned<ServerConnection, TcpStream>;

// How long a blocking read() call waits before giving the writer thread
// a chance to grab the lock. Lower = snappier outgoing messages, more CPU.
const READ_TIMEOUT: Duration = Duration::from_millis(200);

fn close(ws: &mut WebSocket<TlsStream>, reason: &str, code: CloseCode) {
    ws.close(Some(CloseFrame {
        code,
        reason: reason.to_string().into(),
    })).ok();
}

/// True if this tungstenite error is just "no data within the read timeout" —
/// i.e. not a real failure, just our cue to loop back and check for outgoing messages.
fn is_timeout(err: &tungstenite::Error) -> bool {
    matches!(
        err,
        tungstenite::Error::Io(e)
            if e.kind() == io::ErrorKind::WouldBlock || e.kind() == io::ErrorKind::TimedOut
    )
}

pub fn run(db_cookies: sled::Db, rustls_config: Arc<ServerConfig>) {
    let clients: Clients = Arc::new(Mutex::new(HashMap::new()));
    let ws_clients = Arc::clone(&clients);
    thread::spawn(move || init_websocket(ws_clients, db_cookies.clone(), rustls_config));
}

fn init_websocket(clients: Clients, db_cookies: sled::Db, rustls_config: Arc<ServerConfig>) -> Result<(), ()> {
    let address = "0.0.0.0:2121";
    let listener =
        TcpListener::bind(address).map_err(|err| eprintln!("WEBSOCKET ERROR: binding: {err}"))?;
    println!("Websocket listening at {address} ...");

    for stream in listener.incoming() {
        let stream = match stream {
            Ok(s) => s,
            Err(e) => {
                eprintln!("WEBSOCKET ERROR: receiving request: {e}");
                continue;
            }
        };

        let clients = Arc::clone(&clients);
        let db_cookies = db_cookies.clone();
        let rustls_config = Arc::clone(&rustls_config);

        thread::spawn(move || {
            // No read timeout yet — let the TLS + WS handshake run as a normal
            // blocking call. We apply the timeout afterwards, only for the
            // message loop, where we need to periodically release the lock.
            let conn = match ServerConnection::new(rustls_config) {
                Ok(c) => c,
                Err(e) => {
                    eprintln!("WEBSOCKET ERROR: TLS session init: {e}");
                    return;
                }
            };
            let tls_stream: TlsStream = StreamOwned::new(conn, stream);

            handle_client(tls_stream, clients, db_cookies);
        });
    }
    Ok(())
}

fn handle_client(stream: TlsStream, clients: Clients, db_cookies: sled::Db) {
    let mut auth_token: Option<String> = None;

    let callback = |req: &Request, response: Response| {
        if let Some(cookie_header) = req.headers().get("Cookie") {
            if let Ok(cookie_str) = cookie_header.to_str() {
                auth_token = cookie_str
                    .split(';')
                    .map(|kv| kv.trim())
                    .find_map(|kv| {
                        let mut parts = kv.splitn(2, '=');
                        let k = parts.next()?;
                        let v = parts.next()?;
                        (k == "authToken").then(|| v.to_string())
                    });
            }
        }
        Ok(response)
    };

    // Plain blocking handshake — no read timeout is set yet, so this behaves
    // exactly like the TCP-only version: it waits as long as it needs to.
    let mut ws_handshake = match accept_hdr(stream, callback) {
        Ok(ws) => ws,
        Err(err) => {
            eprintln!("WEBSOCKET HANDSHAKE ERROR: {err}");
            return;
        }
    };

    // Now that the handshake is done, apply the read timeout for the message
    // loop below — this is what lets the reader release the mutex periodically
    // so the writer thread can get in.
    if let Err(e) = ws_handshake.get_ref().sock.set_read_timeout(Some(READ_TIMEOUT)) {
        eprintln!("WEBSOCKET ERROR: setting read timeout: {e}");
        return;
    }

    let auth_token = match auth_token {
        Some(t) => t,
        None => {
            eprintln!("WEBSOCKET ERROR: cookie not sent");
            close(&mut ws_handshake, "Please login or register to your account", CloseCode::Error);
            return;
        }
    };

    let client_username = match database::get_username_from_cookie(&db_cookies, &auth_token) {
        Ok(t) => match t {
            Some(u) => u,
            None => {
                eprintln!("WEBSOCKET: Cant find corresponding user for this cookie: {auth_token}");
                close(&mut ws_handshake, "please login or register to your account", CloseCode::Error);
                return;
            }
        },
        Err(e) => {
            eprintln!("WEBSOCKET ERROR: cheching for database cookies: {e}");
            close(&mut ws_handshake, "Websocket error", CloseCode::Error);
            return;
        }
    };
    println!("WEBSOCKET: new connection, auth_token = {auth_token:?}, username = {client_username}");

    let mut ws = ws_handshake;

    let (tx, rx) = channel::<Message>();
    clients.lock().unwrap().insert(auth_token.clone(), tx);

    loop {
        // Flush anything queued for this client before waiting on a read.
        while let Ok(msg) = rx.try_recv() {
            if ws.send(msg).is_err() {
                eprintln!("WEBSOCKET: failed to send queued message, closing");
                clients.lock().unwrap().remove(&auth_token);
                return;
            }
        }

        let msg_recv = match ws.read() {
            Ok(m) => m,
            Err(e) if is_timeout(&e) => {
                // no data within READ_TIMEOUT — loop back and re-check rx
                continue;
            }
            Err(e) => {
                eprintln!("WEBSOCKET: session terminated: {e}");
                break;
            }
        };

        if msg_recv.is_text() || msg_recv.is_binary() {
            let msg: MessageRcv = match json::json_from_slice(Default::default(), &msg_recv.into_data()) {
                Ok(t) => t,
                Err(e) => {
                    eprintln!("WEBSOCKET ERROR: message deserialize: {e}");
                    continue;
                }
            };

            let msg_to_forward_for_recv = Message::text(serde_json::json!({
                "message": msg.message,
                "username": client_username,
            }).to_string());

            let msg_to_forward_for_sender = Message::text(serde_json::json!({
                "message": msg.message,
                "username": "You",
            }).to_string());

            let clients_guard = clients.lock().unwrap();

            for (id, sender) in clients_guard.iter() {
                if id == &auth_token {
                    let _ = sender.send(msg_to_forward_for_sender.clone());
                } else {
                    let _ = sender.send(msg_to_forward_for_recv.clone());
                }
            }
        } else if msg_recv.is_close() {
            println!("WEBSOCKET: Client closed connection");
            break;
        }
    }

    println!("WEBSOCKET: client {auth_token} removed");
    clients.lock().unwrap().remove(&auth_token);
}
