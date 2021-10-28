use http::method::Method;
use server::Server;

mod http; // so that we can use the http module(as it's a folder so we created a mod.rs file for declaring the interfaces)
mod server; // so that we can use the server module

fn main() {
    let server = Server::new("127.0.0.1:8080".to_string());
    server.run();
}
