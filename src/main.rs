use nginx::server::server::Server;
use nginx::config::config::Config;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut config_path = String::new();
    if args.len() > 1 {
        config_path = args[1].clone();
    }
    let cfg = Config::new(config_path);
    let server = Server::new(cfg.document_root, cfg.thread_limit, "0.0.0.0:80".to_string());
    server.serve();
}
