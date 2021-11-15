use crate::http::Request;
use crate::http::Response;
use crate::http::StatusCode;
use std::convert::TryFrom;
use std::io::{Read, Write};
use std::net::TcpListener;
pub struct Server {
    host: String,
    port: String,
}

impl Server {
    pub fn new(host: String, port: String) -> Self {
        Self { host, port }
    }

    pub fn run(self) {
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
                                Ok(request) => {
                                    dbg!(request);
                                    Response::new(StatusCode::Ok, Some("Hiii".to_string()))
                                }
                                Err(e) => {
                                    println!("Failed {}", e);
                                    Response::new(StatusCode::BadRequest, None)
                                }
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
