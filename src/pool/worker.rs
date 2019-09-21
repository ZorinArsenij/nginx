use crate::http::{request, response};
use std::fs::File;
use std::io::{Read, Write};
use std::net;
use std::path::Path;
use std::sync::{mpsc, Arc, Mutex};
use std::thread;

pub struct Worker {
    pub id: usize,
    pub thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    // new create inctance of worker
    pub fn new(
        id: usize,
        root: String,
        receiver: Arc<Mutex<mpsc::Receiver<net::TcpStream>>>,
    ) -> Worker {
        let thread = thread::spawn(move || {
            let mut counter = 0;
            loop {
                let mut conn = receiver.lock().unwrap().recv().unwrap();

                // Debug
                counter = counter + 1;
                println!("Worker id {} receive {} request", id, counter);
                //

                let req = match self::Worker::read_from_conn(&mut conn) {
                    Ok(req) => {
                        println!("Req: {:?}", req);
                        req
                    }
                    Err(e) => {
                        println!("Error: {}", e);
                        continue;
                    }
                };
                let r = root.clone();
                let resp = self::Worker::get_resp_from_req(r, req);

                conn.write(
                    format!(
                        "HTTP/1.1 {} OK
Date: Mon, 27 Jul 2009 12:28:53 GMT
Server: Apache/2.2.14 (Win32)
Last-Modified: Wed, 22 Jul 2009 19:15:56 GMT
Content-Length: 0
Content-Type: text/html",
                        resp.status_code
                    )
                    .as_bytes(),
                )
                .unwrap();
                conn.flush().unwrap();
            }
        });

        Worker {
            id: id,
            thread: Some(thread),
        }
    }

    fn read_from_conn(conn: &mut net::TcpStream) -> request::Result<request::Request> {
        let mut buf = [0; 2048];

        match conn.read(&mut buf) {
            Ok(readed) => {
                if readed == 0 {
                    return Err(request::ParseError);
                }
            }
            Err(e) => {
                println!("Reading from connection failed: {}", e);
                return Err(request::ParseError);
            }
        }

        match request::parse(&buf) {
            Ok(req) => {
                println!("Receive request {:?}", req);
                Ok(req)
            }
            Err(e) => {
                println!("Reading from connection failed: {}", e);
                Err(request::ParseError)
            }
        }

        // println!("Receive {}", String::from_utf8(buf.to_vec()).unwrap());
    }

    fn get_resp_from_req(root: String, req: request::Request) -> response::Response {
        let mut resp = response::Response::new();
        let absolute_path = format!("{}{}", root, req.path);
        let path = Path::new(&absolute_path);

        let path_buf = match path.is_dir() {
            true => {
                resp.status_code = response::FORBIDDEN;
                path.join("index.html")
            }
            false => path.to_path_buf(),
        };

        let filepath = match path_buf.canonicalize() {
            Ok(path) => path,
            Err(_) => {
                resp.status_code = response::NOT_FOUND;
                return resp;
            }
        };

        match req.method.as_str() {
            "GET" => match File::open(filepath) {
                Ok(file) => {
                    resp.data = Some(file);
                }
                Err(_) => {
                    resp.status_code = response::NOT_FOUND;
                }
            },
            "HEAD" => {
                if !path.exists() {
                    resp.status_code = response::NOT_FOUND;
                }
            }
            _ => {
                resp.status_code = response::NOT_ALLOWED;
            }
        }
        resp
    }

    fn write_to_conn(conn: &mut net::TcpStream) {}
}
