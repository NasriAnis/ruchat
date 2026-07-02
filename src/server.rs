use tiny_http::{Request, Response};
use std::fs::File;
use std::path::Path;
use std::io::Read;

use crate::json::get_from_json;


// Wrappers arround tiny_http::Request
pub trait RequestExt {
    fn serve_file(self, path: &str, content_type: &str);
    fn serve_404(self);
    fn respond_with<R: Read>(self, response: Response<R>);
    fn is_websock_upgrade(&self) -> bool;
}
impl RequestExt for Request {
    fn serve_file(self, path: &str, content_type: &str) {
        let mut response = Response::from_file(
            File::open(Path::new(path))
             .expect("File to server not found")
        );
        response = response.with_header(
            tiny_http::Header::from_bytes("Content-Type", content_type)
             .expect("Uncorrect header")
        );

        self.respond_with(response);
    }

    fn serve_404(self) {
        let response = Response::empty(404);
        self.respond_with(response);
    }

    fn respond_with<R: Read>(self, response: Response<R>){
        match self.respond(response){
            Ok(_t) => {},
            Err(e) => {
                eprintln!("ERROR in responding: {e}");
            }
        };
    }

    fn is_websock_upgrade(&self) -> bool {
        let mut is_ws_upgrade = false;

        for h in self.headers() {
            let field = h.field.as_str().as_str();
            let value = h.value.as_str();
            
            if field.eq_ignore_ascii_case("Upgrade") && value.eq_ignore_ascii_case("websocket") {
                is_ws_upgrade = true;
            }
        }
        return is_ws_upgrade;
    }
}

pub fn handle_msg_api(mut request: Request){
    let mut req_body = String::new();
    let req_as_reader = request.as_reader();
    match req_as_reader.read_to_string(&mut req_body){
        Ok(_t) => {},
        Err(e) => eprintln!("ERROR (api): {e}"),
    };

    match get_from_json(req_body){
        Some(t) => {
            println!("MESSAGE: {message}", message = t.message);
            let response = Response::empty(200);
            request.respond_with(response);
        }
        None => {
            eprintln!("ERROR: Bad request body");
            let response = Response::empty(400);
            request.respond_with(response);
        }
    };
}
