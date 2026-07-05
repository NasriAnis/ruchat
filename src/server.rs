use tiny_http::{Request, Response, Server, Method};

use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::thread;

// Wrappers arround tiny_http::Request
trait RequestExt {
    fn serve_file(self, path: &str, content_type: &str);
    fn serve_404(self);
    fn respond_with<R: Read>(self, response: Response<R>);
}

impl RequestExt for Request {
    fn serve_file(self, path: &str, content_type: &str) {
        let mut response =
            Response::from_file(File::open(Path::new(path)).expect("File to server not found"));
        response = response.with_header(
            tiny_http::Header::from_bytes("Content-Type", content_type).expect("Uncorrect header"),
        );

        self.respond_with(response);
    }

    fn serve_404(self) {
        let response = Response::empty(404);
        self.respond_with(response);
    }

    fn respond_with<R: Read>(self, response: Response<R>) {
        match self.respond(response) {
            Ok(_t) => {}
            Err(e) => {
                eprintln!("ERROR in responding: {e}");
            }
        };
    }
}

pub fn run(){
    let address = "0.0.0.0:2020";
    let server = match Server::http(address) {
        Ok(t) => t,
        Err(e) => {
            eprintln!("ERROR: {e}");
            panic!();
        }
    };
    println!("Server listening at {address} ...");
    
    thread::spawn(move || {
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
                    request.serve_file("public/indexv2.html", "text/html; charset=utf-8");
                }
                ("/chat", Method::Get) => {
                    request.serve_file("public/chatv2.html", "text/html; charset=utf-8");
                }
                _ => {
                    request.serve_404();
                }
            }
        };
    });
}
