fn main() {
    let server = Server::new("127.0.0.1".to_string(), "8080".to_string());
    server.run();
}

struct Server {
    host: String,
    port: String,
}

impl Server {
    fn new(host: String, port: String) -> Self {
        Self { host, port }
    }

    fn run(self) {
        println!("Running on {}:{}", self.host, self.port)
    }
}
