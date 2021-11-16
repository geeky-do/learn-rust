use crate::http::{ParseError, Request, Response, StatusCode};
use std::convert::TryFrom;
use std::io::{Read, Write};
use std::net::TcpListener;
pub struct Server {
    host: String,
    port: String,
}
pub trait Handler {
    fn handle_request(&mut self, request: &Request) -> Response {
        dbg!(request);
        Response::new(StatusCode::Ok, Some("<h1>Hiii</h1>".to_string()))
    }
    fn handle_bad_request(&mut self, e: &ParseError) -> Response {
        println!("Failed {}", e);
        Response::new(StatusCode::BadRequest, None)
    }
}
impl Server {
    pub fn new(host: String, port: String) -> Self {
        Self { host, port }
    }

    pub fn run(self, handler: impl Handler) {
        let address = format!("{}:{}", &self.host, &self.port);
        println!("Running on {}", address);
        let listener = TcpListener::bind(&address).unwrap();
        loop {
            match listener.accept() {
                Ok((mut stream, addr)) => {
                    println!("OK @{}", addr);
                    let mut buffer = [0; 1024];
                    match stream.read(&mut buffer) {
                        Ok(_) => {
                            println!("Received a frequest: {}", String::from_utf8_lossy(&buffer));
                            let response = match Request::try_from(&buffer[..]) {
                                Ok(request) => handler.handle_request(&request),
                                Err(e) => handler.handle_bad_request(&e),
                            };
                            if let Err(e) = response.send(&mut stream) {
                                println!("Failed to send response: {}", e)
                            }
                        }
                        Err(e) => println!("Err {}", e),
                    }
                }
                Err(e) => println!("Err {}", e),
            }
        }
    }
}
