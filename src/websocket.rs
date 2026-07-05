use std::collections::HashMap;
use std::net::{TcpListener, TcpStream};
use std::sync::mpsc::{Sender, channel};
use std::sync::{Arc, Mutex};
use std::thread;

use tungstenite::accept;
use tungstenite::protocol::{Message, WebSocket};

type Clients = Arc<Mutex<HashMap<u64, Sender<Message>>>>;

pub fn websocket() {
    let clients: Clients = Arc::new(Mutex::new(HashMap::new()));
    let ws_clients = Arc::clone(&clients);
    thread::spawn(move || init_websocket(ws_clients));
}

pub fn init_websocket(clients: Clients) -> Result<(), ()> {
    let address = "0.0.0.0:2121";
    let listener =
        TcpListener::bind(address).map_err(|err| eprintln!("WEBSOCKET ERROR: binding: {err}"))?;
    println!("Websocket listening at {address} ...");

    let mut next_id = 0u64;

    for stream in listener.incoming() {
        let stream = match stream {
            Ok(s) => s,
            Err(e) => {
                eprintln!("WEBSOCKET ERROR: receiving request: {e}");
                continue;
            }
        };
        let clients = Arc::clone(&clients);
        let id = next_id;
        next_id += 1;

        thread::spawn(move || {
            handle_client(stream, clients, id);
        });
    }
    Ok(())
}

pub fn handle_client(stream: TcpStream, clients: Clients, id: u64) {
    let mut ws_handshake = match accept(stream) {
        Ok(ws) => ws,
        Err(err) => {
            eprintln!("WEBSOCKET HANDSHAKE ERROR: {err}");
            return;
        }
    };

    let write_stream = ws_handshake.get_ref().try_clone().unwrap();
    let mut ws_write =
        WebSocket::from_raw_socket(write_stream, tungstenite::protocol::Role::Server, None);
    let (tx, rx) = channel::<Message>();
    clients.lock().unwrap().insert(id, tx);

    thread::spawn(move || {
        for msgs in rx {
            if ws_write.send(msgs).is_err() {
                eprintln!("WEBSOCKET: Thread cant send message exiting");
                break;
            };
            // println!("WEBSOCKET: Sent succesfully");
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
        // println!("WEBSOCKET: new session");

        if msg.is_text() || msg.is_binary() {
            // println!("WEBSOCKET: Received: {}", msg);
            let clients_guard = clients.lock().unwrap();
            for (_id, sender) in clients_guard.iter() {
                let _ = sender.send(msg.clone());
            }
        } else if msg.is_close() {
            // println!("WEBSOCKET: Client closed connection");
            break;
        }
    }

    clients.lock().unwrap().remove(&id);
    // println!("WEBSOCKET: client {id} removed");
}
