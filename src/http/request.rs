use super::method::Method; // using the super as request belongs to the same folder as requests

pub struct Request {
  path: String,
  query_string: Option<String>,
  method: Method,
}
