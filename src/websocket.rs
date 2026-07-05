use std::net::{
    TcpStream,
    TcpListener
};
use std::thread;
use std::sync::mpsc::channel;

use tungstenite::protocol::{
    WebSocket,
    Message
};
use tungstenite::accept;

pub fn init_websocket() -> Result<(), ()>{

    let address = "0.0.0.0:2121";
    let listener = TcpListener::bind(address).map_err(|err| {
        eprintln!("WEBSOCKET ERROR: binding: {err}")
    })?;
    println!("Websocket listening at {address} ...");

    for stream in listener.incoming() {
        let stream = match stream {
            Ok(s) => s,
            Err(e) => {
                eprintln!("WEBSOCKET ERROR: receiving request: {e}");
                continue
            }
        };
        thread::spawn(||{
            handle_client(stream);
        });

    };
    Ok(())
}

pub fn handle_client(stream: TcpStream){
    let mut ws_handshake = match accept(stream) {
        Ok(ws) => ws,
        Err(err) => {
            eprintln!("WEBSOCKET HANDSHAKE ERROR: {err}");
            return;
        }
    };

    let write_stream = ws_handshake.get_ref().try_clone().unwrap();
    let mut ws_write = WebSocket::from_raw_socket(
        write_stream,
        tungstenite::protocol::Role::Server,
        None,
    );
    let (tx, rx) = channel();

    thread::spawn( move ||{
        for msgs in rx {
            if ws_write.send(msgs).is_err(){
                break;
            };
            println!("Sent succesfully");
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
            println!("WEBSOCKET: Received: {}", msg);
            tx.send(msg).unwrap();
        }
        else if msg.is_close() {
            println!("WEBSOCKET: Client closed connection");
            break;
        }
    }

}
