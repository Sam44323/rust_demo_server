// exposing the module from single point mod by convention
pub mod method;
pub mod request;

// example of exposing the module specifics from the file
pub use method::Method;
pub use request::Request;
