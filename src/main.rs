use nginx::server::server::Server;

fn main() {
    let server = Server::new(String::from("/var/www/html"), 256, "0.0.0.0:80".to_string());
    server.serve();
}
