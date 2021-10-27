fn main() {
    let string = String::from("127.0.0.1:8080");
    let string_slice = &string[10..]; // example of creating a slice of the string
    println!("{}", string_slice);

    // let server = Server::new("127.0.0.1:8080".to_string());
    // server.run();
}

struct Server {
    addr: String,
}

impl Server {
    fn new(addr: String) -> Self {
        Server { addr }
    }

    fn run(self) {
        println!("{}", self.addr);
    }
}
