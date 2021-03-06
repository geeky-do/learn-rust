use super::http::{Method, ParseError, Request, Response, StatusCode};
use super::server::Handler;
use std::fs;
pub struct RequestHandler {
    public_path: String,
}

impl RequestHandler {
    pub fn new(public_path: String) -> Self {
        Self { public_path }
    }
    fn read_file(&self, file_path: &str) -> Option<String> {
        let path = format!("{}{}", self.public_path, file_path);
        fs::read_to_string(path).ok()
    }
}

impl Handler for RequestHandler {
    fn handle_request(&mut self, request: &Request) -> Response {
        dbg!(request);
        match request.method() {
            Method::GET => Response::new(StatusCode::Ok, self.read_file("index.html")),
            Method::POST => Response::new(StatusCode::Ok, Some("<h1>POST</h1>".to_string())),
            Method::DELETE => Response::new(StatusCode::Ok, Some("<h1>DELETE</h1>".to_string())),
            Method::PUT => Response::new(StatusCode::Ok, Some("<h1>PUT</h1>".to_string())),
        }
    }
}
