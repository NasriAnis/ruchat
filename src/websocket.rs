use std::collections::HashMap;
use std::net::{TcpListener, TcpStream};
use std::sync::mpsc::{Sender, channel};
use std::sync::{Arc, Mutex};
use std::thread;

use tungstenite::accept_hdr;
use tungstenite::handshake::server::{Request, Response};
use tungstenite::protocol::{Message, WebSocket};

use serde::{Deserialize, Serialize};

use crate::database;
use crate::json;

#[derive(Serialize, Deserialize, Debug, Default)]
struct MessageRcv {
    message: String,
}

type Clients = Arc<Mutex<HashMap<String, Sender<Message>>>>;

pub fn run(db_cookies: sled::Db) {
    let clients: Clients = Arc::new(Mutex::new(HashMap::new()));
    let ws_clients = Arc::clone(&clients);
    thread::spawn(move || init_websocket(ws_clients, db_cookies.clone()));
}

fn init_websocket(clients: Clients, db_cookies: sled::Db) -> Result<(), ()> {
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
        let value = db_cookies.clone();

        thread::spawn(move || {
            handle_client(stream, clients, value);
        });
    }
    Ok(())
}

fn handle_client(stream: TcpStream, clients: Clients, db_cookies: sled::Db) {
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

    // todo: Need better handling
    let client_username = match database::get_username_from_cookie(&db_cookies, &auth_token){
        Ok(t) => match t {
            Some(u) => u,
            None => {
                eprintln!("WEBSOCKET: Cant find corresponding user for this cookie: {auth_token}");
                return
            },
        }
        Err(e) => {
            eprintln!("WEBSOCKET ERROR: cheching for database cookies: {e}");
            return
        }
    };
    println!("WEBSOCKET: new connection, auth_token = {auth_token:?}, username = {client_username}");

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
        let msg_recv = match ws_handshake.read() {
            Ok(t) => t,
            Err(e) => {
                eprintln!("WEBSOCKET: session terminated: {}", e);
                break;
            }
        };
        println!("WEBSOCKET: new session");

        if msg_recv.is_text() || msg_recv.is_binary() {
            let mut msg: MessageRcv = Default::default(); 
            msg = match json::json_from_slice(msg, &msg_recv.into_data()){
                Ok(t) => t,
                Err(e) => {
                    eprintln!("WEBSOCKET ERROR: message deserialize: {e}");
                    continue;
                }
            };

            // send request with username depending of who send it
            // and if you or another sent it
            // the javascript will watch for this in the frontend
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
                // todo: handle the message
                if id == &auth_token{
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

    println!("WEBSOCKET: client {auth_token} removed", auth_token = auth_token.clone());
    clients.lock().unwrap().remove(&auth_token);
}
