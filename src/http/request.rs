use super::method::Method; // using the super as request belongs to the same folder as requests
use std::convert::TryFrom;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};
use std::str; // using a special Result type used for Display Trait
use std::str::Utf8Error;

pub struct Request {
  path: String,
  query_string: Option<String>,
  method: Method,
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

/**
 * we are implementing Debug and Display traits for our ParseError type so
 * that we can print the error message in a nice way and make the Error trait work
 */

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

impl From<Utf8Error> for ParseError {
  fn from(_: Utf8Error) -> Self {
    return Self::InvalidEncoding;
  }
}

// Adding custom Error implementation for the ParseError enum
impl Error for ParseError {}

/**
 * example of implementing a trait for struct. As here we'll be converting a byte slice to the Request type, so we passed the u8 generic value to the TryFrom trait
 */

impl TryFrom<&[u8]> for Request {
  type Error = ParseError; // adding a concrete type to the Error type alias

  fn try_from(buff: &[u8]) -> Result<Self, Self::Error> {
    /**
     * This line of code checks the transformation for utf-8 encoding to string. If it can, then it return the string, else it returns the error(here it tries to match the error as defined in the generics, for example, for this case it's ParseError)
     */
    let request = str::from_utf8(buff)?; // converting the byte slice to a string
    unimplemented!()
  }
}

// We are using Option because if the query string is empty, we return None
// here we return the word and the rest string slice
fn get_next_word(request: &str) -> Option<(&str, &str)> {
  let mut iter = request.chars();
  for (index, value) in iter.enumerate() {
    if value == ' ' {
      return Some((&request[..index], &request[index + 1..]));
    }
  }
  None // returning None if the string is exhausted or empty
}
