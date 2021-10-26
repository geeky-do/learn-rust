use super::method::Method;
pub struct Request {
    resource_name: String,
    query_param: Option<String>,
    method: Method,
}
