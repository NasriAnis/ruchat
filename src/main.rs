use tiny_http::{Server, Response, Method};

mod server;
use crate::server::RequestExt;

fn main() {
    let address = "0.0.0.0:2020";
    let server = match Server::http(address){
        Ok(t) => t,
        Err(e) => {
            eprintln!("ERROR: {e}");
            panic!();
        }
    };
    println!("Server listenning at {address} ...");
    loop {
        let mut request = match server.recv() {
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
                request.serve_file("public/frontend.html", "text/html; charset=utf-8")
            }
            ("/message", Method::Post) => {
                let req_as_reader = request.as_reader();
                let mut req_body = String::new();
                match req_as_reader.read_to_string(&mut req_body){
                    Ok(_t) => println!("Data: {req_body}"),
                    Err(e) => eprintln!("ERROR (api): {e}"),
                };

                let response = Response::empty(200);
                request.respond_with(response);
            }
            _ => {
                request.serve_404();
            }
        }
    }
}
