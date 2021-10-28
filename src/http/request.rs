mod method;

pub mod request {
  pub struct Request {
    path: String,
    query_string: Option<String>,
    method: Method,
  }
}
