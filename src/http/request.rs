use super::method::Method; // using the super as request belongs to the same folder as requests
use std::convert::TryFrom;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter, Result as FmtResult}; // using a special Result type used for Display Trait

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

pub enum ParseError {
  InvalidRequest,
  InvalidEncoding, // when the encoding is not utf-8
  InvalidProtocol, // if the HTTP protocol is not 1.1
  InvalidMethod,   // if the method is not a valid HTTP method
}

impl ParseError {
  fn message(&self) -> &str {
    match self {
      Self::InvalidRequest => "Invalid Request",
      Self::InvalidEncoding => "Invalid Encoding",
      Self::InvalidProtocol => "Invalid Protocol",
      Self::InvalidMethod => "Invalid Method",
    }
  }
}

// implementing the Display trait for the ParseError enum
impl Display for ParseError {
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    // writing the string representation of the enum(message) to the formatter
    write!(f, "{}", self.message())
  }
}

// implementing the Debug trait for the ParseError enum
impl Debug for ParseError {
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    write!(f, "{}", self.message())
  }
}
impl Error for ParseError {}
