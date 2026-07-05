use tiny_http::{Server, Method};
use crate::server::{
    RequestExt,
    handle_msg_api,
    // handle_websocket
};

use crate::websocket::websocket;

use std::thread;

pub mod json;
mod websocket;
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

    thread::spawn(|| {websocket(); });

    loop {
        let request = match server.recv() {
            Ok(rq) => rq,
            Err(e) => {
                eprintln!("ERROR: {}", e);
                continue;
            }
        };
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
