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
                self::Worker::write_to_conn(&mut conn, resp);
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

        let filepath = (match path_buf.canonicalize() {
            Ok(path) => path,
            Err(_) => {
                resp.status_code = response::NOT_FOUND;
                return resp;
            }
        })
        .clone();

        match req.method.as_str() {
            "GET" => match File::open(&filepath) {
                Ok(file) => {
                    let lenght = file.metadata().unwrap().len();
                    let ext = filepath.extension().unwrap().to_str().unwrap();
                    resp.add_content_lenght(lenght);
                    resp.add_content_type(ext);
                    resp.data = Some(file);
                }
                Err(_) => {
                    resp.status_code = response::NOT_FOUND;
                }
            },
            "HEAD" => {
                if path.exists() {
                    let lenght = path.metadata().unwrap().len();
                    let ext = filepath.extension().unwrap().to_str().unwrap();
                    resp.add_content_lenght(lenght);
                    resp.add_content_type(ext);
                } else {
                    resp.status_code = response::NOT_FOUND;
                }
            }
            _ => {
                resp.status_code = response::NOT_ALLOWED;
            }
        }
        resp
    }

    fn write_to_conn(conn: &mut net::TcpStream, resp: response::Response) {
        conn.write(resp.to_string().as_bytes()).unwrap();
        conn.flush().unwrap();
    }
}
