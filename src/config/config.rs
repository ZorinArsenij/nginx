use std::fs;
use std::path::Path;

const DEFAULT_THREAD_LIMIT: usize = 256;
const DEFAULT_DOCUMENT_ROOT: &str = "/var/www/html";

const THREAD_LIMIT: &str = "thread_limit";
const DOCUMENT_ROOT: &str = "document_root";

pub struct Config {
    pub thread_limit: usize,
    pub document_root: String,
}

impl Config {
    pub fn new(path: String) -> Config {
        let mut cfg = Config {
            thread_limit: DEFAULT_THREAD_LIMIT,
            document_root: String::from(DEFAULT_DOCUMENT_ROOT),
        };

        let filepath = Path::new(&path);
        if filepath.is_dir() || !filepath.exists() {
            return cfg;
        }

        let data = fs::read_to_string(filepath).unwrap();
        let lines: Vec<&str> = data.split("\n").collect();
        for line in &lines {
            if line.contains(THREAD_LIMIT) {
                let parts: Vec<&str> = line.trim().split(" ").collect();
                if parts.len() == 2 {
                    match parts[1].parse::<usize>() {
                        Ok(value) => {
                            cfg.thread_limit = value;
                        }
                        Err(_) => {}
                    }
                }
            } else if line.contains(DOCUMENT_ROOT) {
                let parts: Vec<&str> = line.trim().split(" ").collect();
                if parts.len() == 2 {
                    cfg.document_root = String::from(parts[1]);
                }
            }
        }
        cfg
    }
}
