pub struct Response {
    status_code: StatusCode,
    body: Option<String>,
}

pub enum StatusCode {}

impl Response {
    pub fn new(status_code: StatusCode, body: Option<String>) -> Self {
        Response { status_code, body }
    }
}
