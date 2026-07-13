pub mod database;
pub mod json;
mod websocket;
mod server;

fn main() {
    let dbs = database::init();
    server::run(dbs.clone());
    websocket::run(dbs.cookies);
    std::thread::park();
}
