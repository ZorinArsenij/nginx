use crate::pool::pool::Pool;
use std::net;
use std::time::Duration;

const TIMEOUT: Duration = Duration::from_secs(3);

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
        let mut count = 0;
        for conn in self.listener.incoming() {
            let connection = conn.unwrap();
            match connection.set_read_timeout(Some(TIMEOUT)) {
                Err(_) => {
                    continue;
                }
                _ => {}
            }
            count = count + 1;
            println!("Receive request {}", count);
            self.pool.process_request(connection);
            println!("Done request {}", count);
        }
    }
}
