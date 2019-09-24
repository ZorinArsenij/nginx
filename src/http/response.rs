use chrono::prelude::Utc;
use std::collections::HashMap;
use std::fs;
use std::io::{Read, Seek, SeekFrom, Write};
use std::net;

pub const OK: u32 = 200;
pub const FORBIDDEN: u32 = 403;
pub const NOT_FOUND: u32 = 404;
pub const NOT_ALLOWED: u32 = 405;

const CONTENT_LENGHT: &str = "Content-Length";
const CONTENT_TYPE: &str = "Content-Type";
const DATE: &str = "Date";
const SERVER: &str = "Server";
const CONNECTION: &str = "Connection";

const HTML: &str = "html";
const CSS: &str = "css";
const JS: &str = "js";
const JPG: &str = "jpg";
const JPEG: &str = "jpeg";
const PNG: &str = "png";
const GIF: &str = "gif";
const SWF: &str = "swf";

static CRLF: &str = "\r\n";

pub struct Response {
    pub status_code: u32,
    pub headers: HashMap<String, String>,
    pub data: Option<fs::File>,
}

impl Response {
    pub fn new() -> Response {
        let mut headers = HashMap::new();
        headers.insert(
            String::from(DATE),
            Utc::now().format("%a, %d %b %Y %H:%M:%S GMT").to_string(),
        );
        headers.insert(String::from(SERVER), String::from("Rust NGINX"));
        headers.insert(String::from(CONNECTION), String::from("close"));
        Response {
            status_code: OK,
            headers: headers,
            data: None,
        }
    }

    pub fn add_content_lenght(&mut self, lenght: u64) {
        self.headers
            .insert(String::from(CONTENT_LENGHT), lenght.to_string());
    }

    pub fn add_content_type(&mut self, ext: &str) {
        let content_type = match ext {
            HTML => "text/html",
            CSS => "text/css",
            JS => "application/javascript",
            JPG | JPEG => "image/jpeg",
            PNG => "image/png",
            GIF => "image/gif",
            SWF => "application/x-shockwave-flash",
            _ => "application/octet-stream",
        };
        self.headers
            .insert(String::from(CONTENT_TYPE), String::from(content_type));
    }

    pub fn write_to_conn(self, conn: &mut net::TcpStream) {
        let mut result = String::new();
        result.push_str(
            format!("{} {} {}{}", "HTTP/1.1", self.status_code, "IAmNginx", CRLF).as_str(),
        );
        for (header, value) in &self.headers {
            result.push_str(format!("{}: {}{}", header, value, CRLF).as_str());
        }
        result.push_str(CRLF);
        conn.write(result.as_bytes()).unwrap();

        match self.data {
            Some(mut data) => {
                let mut buf = [0; 1024 * 1024];
                let mut offset: u64 = 0;
                loop {
                    match data.read(&mut buf).unwrap() {
                        n => {
                            if n == 0 {
                                break;
                            }
                            offset += n as u64;
                            match conn.write(&buf[..n]) {
                                Ok(_) => {}
                                Err(_) => return,
                            }
                            data.seek(SeekFrom::Start(offset)).unwrap();
                        }
                    }
                }
            }
            _ => {}
        }
        conn.flush().unwrap();
    }
}
