use rust_server::http::Server;

fn main() {
    let server = Server::new("127.0.0.1:8000".to_string());
    println!("Hello, world!");

    if let Err(e) = server.run() {
        print!("Error: {}", e);
    }
}
