pub struct Server {
    host: String,
    port: String,
}

impl Server {
    pub fn new(host: String, port: String) -> Self {
        Self { host, port }
    }

    pub fn run(self) {
        println!("Running on {}:{}", self.host, self.port)
    }
}
