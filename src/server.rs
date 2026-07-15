use tiny_http::{Request, Response, Server, Method};

use crate::database::{self, register_user, check_login};
use crate::json;

use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::thread;

// Wrappers arround tiny_http::Request
trait RequestExt {
    fn serve_file(self, path: &str, content_type: &str);
    fn serve(self, statuscode: u16);
    fn serve_cookie(self, cookie: &str);
    // fn serve_bytes(self, data: &[u8], content_type: &str);
    fn get_body(&mut self) -> Result<String, std::io::Error>;
    fn respond_with<R: Read>(self, response: Response<R>);
    fn handle_register(self, db: &database::Databases);
    fn handle_login(self, db: &database::Databases);
}

impl RequestExt for Request {
    fn serve_file(self, path: &str, content_type: &str) {
        let response =
            Response::from_file(File::open(Path::new(path)) .expect("File to server not found"))
                .with_header(
                    tiny_http::Header::from_bytes("Content-Type", content_type) .expect("Uncorrect header"),
                );

        self.respond_with(response);
    }

    // fn serve_bytes(self, data: &[u8], content_type: &str) {
    //     let response = Response::from_data(data)
    //         .with_header(
    //             tiny_http::Header::from_bytes("Content-Type", content_type) .expect("Uncorrect header"),
    //         );
    //     self.respond_with(response);
    // }

    fn serve_cookie(self, cookie: &str) {
        let response = Response::empty(200)
            .with_header(
                tiny_http::Header::from_bytes("Cookie", cookie).expect("Uncorrect header"),
            );
        self.respond_with(response);
    }

    fn serve(self, statuscode: u16) {
        let response = Response::empty(statuscode);
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

    fn get_body(&mut self) -> Result<String, std::io::Error> {
        let mut req_body = String::new();
        let req_as_reader = self.as_reader();
        match req_as_reader.read_to_string(&mut req_body){
            Ok(_t) => {
                return Ok(req_body);
            },
            Err(e) => {
                eprintln!("ERROR (api): {e}");
                return Err(e);
            },
        };
    }

    fn handle_register(mut self, dbs: &database::Databases){
        let req_body = match self.get_body(){
            Ok(t) => t,
            Err(_e) => {
                self.serve(500);
                return;
            }
        };
        
        let mut user: database::User = Default::default();
        user = match json::json_from_slice(user, &req_body.into_bytes()){
            Ok(t) => t,
            Err(e) => {
                eprintln!("Register JSON API: request body error: {e}");
                self.serve(400);
                return;
            }
        };


        match register_user(&dbs.users, &user){
            Ok(_) => {
                let cookie: String = format!("{}@{}", user.username, user.password);
                let full_cookie: String = format!("authToken={}", cookie);
                let _ = match database::save_cookie(&dbs.cookies, &user.username, &cookie){
                    Ok(_) => self.serve_cookie(&full_cookie),
                    Err(e) => {
                        eprintln!("ERROR: Saving cookie: {e}");
                        self.serve(500);
                        return
                    }
                };
            },
            Err(_) => self.serve(500),
        };
    }

    fn handle_login(mut self, dbs: &database::Databases){
        let req_body = match self.get_body(){
            Ok(t) => t,
            Err(_e) => {
                self.serve(500);
                return;
            }
        };

        let mut user: database::User = Default::default();
        user = match json::json_from_slice(user, &req_body.into_bytes()){
            Ok(t) => t,
            Err(e) => {
                eprintln!("LOGIN JSON API: request body error: {e}");
                self.serve(400);
                return;
            }
        };

        let is_loged = match check_login(&dbs.users, &user.username, &user.password){
            Ok(t) => t,
            Err(e) => {
                eprintln!("DATABASE GET ERROR: {e}");
                self.serve(500);
                return
            }
        };

        if is_loged {
            // todo: improve cookies, This is experimental
            let cookie: String = format!("{}@{}", user.username, user.password);
            let full_cookie: String = format!("authToken={}", cookie);
            let _ = match database::save_cookie(&dbs.cookies, &user.username, &cookie){
                Ok(_) => self.serve_cookie(&full_cookie),
                Err(e) => {
                    eprintln!("ERROR: Saving cookie: {e}");
                    self.serve(500);
                    return
                }
            };
        } else {
            self.serve(401)
        }
    }
}

pub fn run(dbs: database::Databases) {
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
                    request.serve_file("public/index.html", "text/html; charset=utf-8");
                }
                ("/chat", Method::Get) => {
                    request.serve_file("public/chat.html", "text/html; charset=utf-8");
                }
                ("/login", Method::Get) => {
                    request.serve_file("public/login.html", "text/html; charset=utf-8");
                }
                ("/register", Method::Get) => {
                    request.serve_file("public/register.html", "text/html; charset=utf-8");
                }

                // APIs
                ("/api/login", Method::Post) => {
                    request.handle_login(&dbs);
                }
                ("/api/register", Method::Post) => {
                    request.handle_register(&dbs);
                }

                // Javascript serve
                ("/js/index.js", Method::Get) => {
                    request.serve_file("public/js/index.js", "text/javascript; charset=utf-8");
                }
                ("/js/chat.js", Method::Get) => {
                    request.serve_file("public/js/chat.js", "text/javascript; charset=utf-8");
                }
                ("/js/register.js", Method::Get) => {
                    request.serve_file("public/js/register.js", "text/javascript; charset=utf-8");
                }
                ("/js/login.js", Method::Get) => {
                    request.serve_file("public/js/login.js", "text/javascript; charset=utf-8");
                }

                // Unkhown endpoint
                _ => {
                    request.serve(404);
                }
            }
        };
    });
}
