use percent_encoding::percent_decode;
use std::error;
use std::fmt;

pub type Result<T> = std::result::Result<T, ParseError>;

#[derive(Debug, Clone)]
pub struct ParseError;

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid request structure")
    }
}

impl error::Error for ParseError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        None
    }
}

#[derive(Debug)]
pub struct Request {
    pub method: String,
    pub path: String,
}

pub fn parse(data: &[u8]) -> Result<Request> {
    let parsed: Vec<&str> = std::str::from_utf8(data)
        .unwrap()
        .split("\r\n")
        .nth(0)
        .unwrap()
        .split(" ")
        .collect();
    if parsed.len() != 3 {
        return Err(ParseError);
    }

    let path_with_args = percent_decode(parsed[1].as_bytes()).decode_utf8().unwrap();
    let path = path_with_args.split("?").nth(0).unwrap();

    Ok(Request {
        method: String::from(parsed[0]),
        path: path.to_string(),
    })
}
