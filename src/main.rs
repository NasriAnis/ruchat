use tiny_http::{Server, Method, Response, Header};
use crate::server::{RequestExt, handle_msg_api};
use tungstenite::protocol::WebSocket;

use std::thread;

pub mod json;
mod server;

fn main() {
    let address = "0.0.0.0:2020";
    let server = match Server::http(address) {
        Ok(t) => t,
        Err(e) => {
            eprintln!("ERROR: {e}");
            panic!();
        }
    };
    println!("Server listening at {address} ...");

    loop {
        let request = match server.recv() {
            Ok(rq) => rq,
            Err(e) => {
                eprintln!("ERROR: {}", e);
                continue;
            }
        };

        if request.is_websock_upgrade() {
            // Check for the key
            let mut sec_websocket_key = None;
            for h in request.headers() {
                let field = h.field.as_str().as_str();
                let value = h.value.as_str();
            
                if field.eq_ignore_ascii_case("Sec-WebSocket-Key") {
                    sec_websocket_key = Some(value.trim().to_string());
                }
            }

            // If with key
            if let Some(key) = sec_websocket_key {
                let accept_key = tungstenite::handshake::derive_accept_key(key.as_bytes());

                let mut response = Response::empty(101)
                    .with_header(Header::from_bytes(&b"Upgrade"[..], &b"websocket"[..]).unwrap())
                    .with_header(Header::from_bytes(&b"Connection"[..], &b"Upgrade"[..]).unwrap())
                    .with_header(Header::from_bytes(&b"Sec-WebSocket-Accept"[..], accept_key.as_bytes()).unwrap());

                let stream = request.upgrade("websocket", response);

                ////////////// Thread Block ////////////////
                thread::spawn(move || {
                    let mut websocket = WebSocket::from_raw_socket(
                        stream, 
                        tungstenite::protocol::Role::Server, 
                        None
                    );

                    println!("WEBSOCKET: connection established");

                    loop {
                        let msg = match websocket.read() {
                            Ok(t) => t,
                            Err(e) => {
                                eprintln!("WEBSOCKET: session terminated: {}", e);
                                break;
                            }
                        };

                        if msg.is_text() || msg.is_binary() {
                            println!("WEBSOCKET: Received: {}", msg);
                            if let Err(e) = websocket.send(msg) {
                                eprintln!("WEBSOCKET: Error sending message: {}", e);
                                break;
                            }
                        } else if msg.is_close() {
                            println!("WEBSOCKET: Client closed connection");
                            break;
                        }
                    }
                });
                ////////////////////////////////////////

                // if whithout key
            } else {
                let response = Response::empty(400);
                let _ = request.respond(response);
            }
            // Standard HTTP Router
        } else {
            println!("RECEIVED: from {remote_address} with {method} at {url}",
                remote_address = match request.remote_addr(){
                    Some(t) => t.to_string(),
                    None => "unknown".to_string(),
                },
                method = request.method(),
                url = request.url(),
            );

            match (request.url(), request.method()) {
                ("/", Method::Get) => {
                    request.serve_file("public/index.html", "text/html; charset=utf-8");
                }
                ("/chat", Method::Get) => {
                    request.serve_file("public/chat.html", "text/html; charset=utf-8");
                }
                ("/api/message", Method::Post) => {
                    handle_msg_api(request);
                }
                _ => {
                    request.serve_404();
                }
            }
        }
    }
}

