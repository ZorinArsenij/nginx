use nginx::server::server::Server;

fn main() {
    let server = Server::new(String::from("./src"), 10, "127.0.0.1:8080".to_string());
    server.serve();
}
