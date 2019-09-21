use std::collections::HashMap;
use std::fs;

pub const OK: u32 = 200;
pub const FORBIDDEN: u32 = 403;
pub const NOT_FOUND: u32 = 404;
pub const NOT_ALLOWED: u32 = 405;

pub struct Response {
    pub status_code: u32,
    pub headers: HashMap<String, String>,
    pub data: Option<fs::File>,
}

impl Response {
    pub fn new() -> Response {
        Response {
            status_code: OK,
            headers: HashMap::new(),
            data: None,
        }
    }
}
