use crate::http::Request;
use std::convert::TryFrom;
use std::io::Read;
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
                            match Request::try_from(&buffer[..]) {
                                Ok(request) => {}
                                Err(e) => println!("Failed {}", e),
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
