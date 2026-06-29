use tiny_http::{Request, Response};
use std::fs::File;
use std::path::Path;
use std::io::Read;

// Wrappers arround tiny_http::Request
pub trait RequestExt {
    fn serve_file(self, path: &str, content_type: &str);
    fn serve_404(self);
    fn respond_with<R: Read>(self, response: Response<R>);
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
}

