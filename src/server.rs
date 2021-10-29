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
      let result = listener.accept();

      if result.is_err() {
        continue;
      }

      let stream = result.unwrap(); // extracting the stream from the result if not error
    }
  }
}
