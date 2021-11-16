use super::http::{Method, ParseError, Request, Response, StatusCode};
use super::server::Handler;
pub struct RequestHandler;

impl Handler for RequestHandler {
    fn handle_request(&mut self, request: &Request) -> Response {
        dbg!(request);
        match request.method() {
            Method::GET => Response::new(StatusCode::Ok, Some("<h1>GET</h1>".to_string())),
            Method::POST => Response::new(StatusCode::Ok, Some("<h1>POST</h1>".to_string())),
            Method::DELETE => Response::new(StatusCode::Ok, Some("<h1>DELETE</h1>".to_string())),
            Method::PUT => Response::new(StatusCode::Ok, Some("<h1>PUT</h1>".to_string())),
        }
    }
}
