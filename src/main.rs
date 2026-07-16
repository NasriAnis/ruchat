pub mod database;
pub mod json;
mod websocket;
mod server;
mod certificates;

fn main() {
    let ((cert_pem, key_pem), rustls_config) = certificates::init(
        "certs/cert.pem", "certs/key.pem"
    );
    let dbs = database::init();
    server::run(dbs.clone(), cert_pem.clone(), key_pem.clone());
    websocket::run(dbs.cookies, rustls_config);
    std::thread::park();
}
