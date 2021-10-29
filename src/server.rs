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
        Ok((_stream, _)) => {
          println!("New connection");
        }
        Err(e) => {
          println!("Failed to establish a connection: {}", e);
        }
      }
    }
  }
}
