use super::http::{ParseError, Request, Response, StatusCode};
use super::server::Handler;
pub struct RequestHandler;

impl Handler for RequestHandler {
    fn handle_request(&mut self, request: &Request) -> Response {
        dbg!(request);
        Response::new(StatusCode::Ok, Some("<h1>Hiii</h1>".to_string()))
    }
}
