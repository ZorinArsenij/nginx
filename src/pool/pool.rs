use super::worker::Worker;
use std::net;
use std::sync::{mpsc, Arc, Mutex};

pub struct Pool {
    _workers: Vec<Worker>,
    sender: mpsc::Sender<net::TcpStream>,
}

impl Pool {
    pub fn new(root: String, cap: usize) -> Pool {
        let (s, r) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(r));
        let mut workers: Vec<Worker> = Vec::with_capacity(cap);
        for id in 0..cap {
            let r = root.clone();
            workers.push(Worker::new(id, r, Arc::clone(&receiver)));
        }

        Pool {
            _workers: workers,
            sender: s,
        }
    }
    pub fn process_request(&self, conn: net::TcpStream) {
        self.sender.send(conn).unwrap();
    }
}
