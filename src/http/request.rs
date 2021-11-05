use super::method::Method; // using the super as request belongs to the same folder as requests
use std::convert::TryFrom;

pub struct Request {
  path: String,
  query_string: Option<String>,
  method: Method,
}

/**
 * example of implementing a trait for struct. As here we'll be converting a byte slice to the Request type, so we passed the u8 generic value to the TryFrom trait
 */

impl TryFrom<&[u8]> for Request {
  type Error = String; // adding a concrete type to the Error type alias

  fn try_from(buff: &[u8]) -> Result<Self, Self::Error> {
    unimplemented!(); // using the unimplemented! macro to indicate that this function is not implemented
  }
}
