use std::str::FromStr;

pub enum Method {
    GET,
    DELETE,
    POST,
    PUT,
}

impl FromStr for Method {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "GET" => Ok(Self::GET),
            "DELETE" => Ok(Self::DELETE),
            "POST" => Ok(Self::POST),
            "PUT" => Ok(Self::PUT),
            _ => Err(String::from("error")),
        }
    }
}

pub struct MethodError;
