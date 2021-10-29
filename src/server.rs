use std::io::Read;
use std::net::TcpListener;

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
              println!("Output: {}", String::from_utf8_lossy(&buffer));
            }
            Err(e) => {
              println!("Failed to read from the connection: {}", e);
            }
          } // taking the input streams and storing it in your buffer(just for our test)
        }
        Err(e) => {
          println!("Failed to establish a connection: {}", e);
        }
      }
    }
  }
}
