pub mod json;
mod websocket;
mod server;

fn main() {
    websocket::run();
    server::run();
    std::thread::park();
}
