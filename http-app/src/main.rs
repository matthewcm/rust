mod server;
mod http;


pub use server::Server;

fn main() {
    let string = String::from("127.0.0.1:8080");
    let server = Server::new(string);
    server.run();
}
