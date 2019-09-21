use crate::pool::pool::Pool;
use std::net;
use std::time::Duration;

const TIMEOUT: Duration = Duration::from_secs(10);

pub struct Server {
    pool: Pool,
    listener: net::TcpListener,
}

impl Server {
    pub fn new(root: String, cap: usize, address: String) -> Server {
        let listener = match net::TcpListener::bind(&address) {
            Ok(l) => l,
            Err(err) => panic!(err),
        };

        Server {
            pool: Pool::new(root, cap),
            listener: listener,
        }
    }

    pub fn serve(&self) {
        for conn in self.listener.incoming() {
            let connection = conn.unwrap();
            connection.set_read_timeout(Some(TIMEOUT)).unwrap();
            self.pool.process_request(connection);
        }
    }
}
