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

    // rust has a special infinite loops
    loop {
      match listener.accept() {
        Ok((_socket, _addr)) => {
          println!("New connection");
        }
        Err(e) => {
          println!("Failed to establish a connection: {}", e);
        }
      }

      // let result = listener.accept();

      // if result.is_err() {
      //   continue;
      // }

      // let (stream, address) = result.unwrap(); // extracting the stream from the result tuple if not error
    }
  }
}
