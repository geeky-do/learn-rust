use http::Method;
use http::Request;
use server::Server;
mod http;
mod server;
fn main() {
    let server = Server::new("127.0.0.1".to_string(), "8080".to_string());
    server.run();
}
