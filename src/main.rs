use tiny_http::{Server, Response};
use std::fs::File;
use std::path::Path;

fn main() {
    let address = "127.0.0.1:2020";
    let server = match Server::http(address){
        Ok(t) => t,
        Err(e) => {
            eprintln!("ERROR: {e}");
            panic!();
        }
    };
    println!("Server listenning at {address} ...");
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

        let front_file = File::open(Path::new("./public/frontend.html")).unwrap();
        let header = tiny_http::Header::from_bytes(&b"Content-Type"[..], &b" text/html; charset=utf-8"[..]).unwrap();
        let mut response = Response::from_file(front_file);
        response = response.with_header(header);
        request.respond(response);
    }
}
