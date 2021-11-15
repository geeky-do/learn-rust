use super::status_code::StatusCode;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::io::{Result as IOResult, Write};
use std::net::TcpStream;

#[derive(Debug)]
pub struct Response {
    status_code: StatusCode,
    body: Option<String>,
}

impl Response {
    pub fn new(status_code: StatusCode, body: Option<String>) -> Self {
        Response { status_code, body }
    }
    pub fn send(&self, stream: &mut TcpStream) -> IOResult<()> {
        let body = match &self.body {
            Some(b) => b,
            None => "",
        };
        write!(
            stream,
            "HTTP/{}{}\r\n\r\n{}",
            self.status_code,
            self.status_code.reason_phrase(),
            body
        )
    }
}
