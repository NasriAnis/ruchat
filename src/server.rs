use tiny_http::{Request, Response, Server, Method};
use crate::database::{self, register_user};
use crate::json;

use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::thread;

// Wrappers arround tiny_http::Request
trait RequestExt {
    fn serve_file(self, path: &str, content_type: &str);
    fn serve_404(self);
    fn respond_with<R: Read>(self, response: Response<R>);
    fn handle_signup(self, db: &sled::Db);
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

    fn handle_signup(mut self, db: &sled::Db){
        let mut req_body = String::new();
        let req_as_reader = self.as_reader();
        match req_as_reader.read_to_string(&mut req_body){
            Ok(_t) => {},
            Err(e) => eprintln!("ERROR (api): {e}"),
        };
        let user_info = match json::user_from_json(req_body){
            Some(t) => t,
            None => {
                eprintln!("SIGN-IN API: request body error"); // todo: need handle
                return;
            }
        };
        let _ = register_user(&db, user_info); // todo: need handle

        let response = Response::empty(200);
        self.respond_with(response);
    }
}

pub fn run(){
    let db = match database::init("/tmp/db"){
        Ok(t) => t,
        Err(e) => {
            eprintln!("DATABASE: Failed to open database: {e}");
            panic!();
        }
    };
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
                // Endpoints
                ("/", Method::Get) => {
                    request.serve_file("public/indexv2.html", "text/html; charset=utf-8");
                }
                ("/chat", Method::Get) => {
                    request.serve_file("public/chatv2.html", "text/html; charset=utf-8");
                }
                ("/login", Method::Get) => {
                    request.serve_file("public/login.html", "text/html; charset=utf-8");
                }
                ("/register", Method::Get) => {
                    request.serve_file("public/register.html", "text/html; charset=utf-8");
                }

                // APIs
                ("/api/login", Method::Post) => {
                    todo!();
                }
                ("/api/register", Method::Post) => {
                    request.handle_signup(&db);
                }

                // Javascript serve
                ("/js/index.js", Method::Get) => {
                    request.serve_file("public/index.js", "text/javascript; charset=utf-8");
                }
                ("/js/chat.js", Method::Get) => {
                    request.serve_file("public/chat.js", "text/javascript; charset=utf-8");
                }
                ("/js/register.js", Method::Get) => {
                    request.serve_file("public/register.js", "text/javascript; charset=utf-8");
                }

                // Unkhown endpoint
                _ => {
                    request.serve_404();
                }
            }
        };
    });
}
