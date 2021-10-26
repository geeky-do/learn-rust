fn main() {
    let get = HTTPMethod::GET;
    let delete = HTTPMethod::DELETE;
    let post = HTTPMethod::POST;
    let put = HTTPMethod::PUT;
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

struct Request {
    resource_name: String,
    query_param: String,
    method: HTTPMethod,
}

enum HTTPMethod {
    GET,
    DELETE,
    POST,
    PUT,
}
