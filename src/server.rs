use crate::http::Request;
use std::convert::{TryFrom, TryInto}; // to use a trait we always have to pull it (even if we are using the function from one of our crates)
use std::io::Read;
use std::net::TcpListener; // accessing the root of the entire crate(i.e. main for our case) file for modules

pub struct Server {
  addr: String,
}

impl Server {
  pub fn new(addr: String) -> Self {
    Server { addr }
  }

  pub fn run(self) {
    println!("Listening on {}", self.addr);
    let listener = TcpListener::bind(&self.addr).unwrap();

    // rust has a special infinite loop
    loop {
      match listener.accept() {
        Ok((mut stream, _)) => {
          let mut buffer = [0; 1024];
          match stream.read(&mut buffer) {
            Ok(_) => {
              println!("Received a request: {}", String::from_utf8_lossy(&buffer));

              // adding a try_from implementation to our buffer
              match Request::try_from(&buffer as &[u8]) {
                Ok(request) => println!("Request parsed successfully"),
                Err(e) => println!("Request could not be parsed: {}", e),
              }
            }
            Err(e) => {
              println!("Failed to read from the connection: {}", e);
            }
          } // taking the input streams and storing it in our buffer(just for our test)
        }
        Err(e) => {
          println!("Failed to establish a connection: {}", e);
        }
      }
    }
  }
}
