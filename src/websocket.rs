use std::collections::HashMap;
use std::net::{TcpListener, TcpStream};
use std::sync::mpsc::{Sender, channel};
use std::sync::{Arc, Mutex};
use std::thread;

use tungstenite::accept_hdr;
use tungstenite::handshake::server::{Request, Response};
use tungstenite::protocol::{Message, WebSocket};

type Clients = Arc<Mutex<HashMap<String, Sender<Message>>>>;

pub fn run() {
    let clients: Clients = Arc::new(Mutex::new(HashMap::new()));
    let ws_clients = Arc::clone(&clients);
    thread::spawn(move || init_websocket(ws_clients));
}

fn init_websocket(clients: Clients) -> Result<(), ()> {
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

        println!("Stream: {stream:?}");

        let clients = Arc::clone(&clients);

        thread::spawn(move || {
            handle_client(stream, clients);
        });
    }
    Ok(())
}

fn handle_client(stream: TcpStream, clients: Clients) {
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

    let mut ws_handshake = match accept_hdr(stream, callback) {
        Ok(ws) => ws,
        Err(err) => {
            eprintln!("WEBSOCKET HANDSHAKE ERROR: {err}");
            return;
        }
    };

    let auth_token = match auth_token {
        Some(t) => t,
        None => {
            eprintln!("WEBSOCKET ERROR: cookie not sent");
            return
        },
    };

    println!("WEBSOCKET: new connection, auth_token = {auth_token:?}");

    let write_stream = ws_handshake.get_ref().try_clone().unwrap();
    let mut ws_write =
        WebSocket::from_raw_socket(write_stream, tungstenite::protocol::Role::Server, None);
    let (tx, rx) = channel::<Message>();
    clients.lock().unwrap().insert(auth_token.clone(), tx);

    thread::spawn(move || {
        for msgs in rx {
            if ws_write.send(msgs).is_err() {
                eprintln!("WEBSOCKET: Thread cant send message exiting");
                break;
            };
        }
    });

    loop {
        let msg = match ws_handshake.read() {
            Ok(t) => t,
            Err(e) => {
                eprintln!("WEBSOCKET: session terminated: {}", e);
                break;
            }
        };
        println!("WEBSOCKET: new session");

        if msg.is_text() || msg.is_binary() {
            let clients_guard = clients.lock().unwrap();
            for (_id, sender) in clients_guard.iter() {
                let _ = sender.send(msg.clone());
            }
        } else if msg.is_close() {
            println!("WEBSOCKET: Client closed connection");
            break;
        }
    }

    println!("WEBSOCKET: client {auth_token} removed", auth_token = auth_token.clone());
    clients.lock().unwrap().remove(&auth_token);
}
