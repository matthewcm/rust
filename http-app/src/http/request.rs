use super::method::Method;
use std::{convert::TryFrom, error::Error, fmt::{ Display, Debug, Formatter, Result as FmtResult}};
use std::str::{Utf8Error};

pub struct Request {
    path: String,
    query_string: Option<String>,
    method: Method,
}

impl Request {

}

impl TryFrom<&[u8]> for Request {
    type Error = ParseError;

    // GET /search?name=abc&sort=1 HTTP/1.1
    fn try_from(buf: &[u8]) -> Result<Self, Self::Error>{

        let request = str::from_utf8(buf)?;

        // match str::from_utf8(buf) {
        //     Ok(request) => {},
        //     Err(_) => Err(ParseError::InvalidEncoding),
        // }

        // str::from_utf8(buf).or(Err(ParseError::InvalidEncoding)) {
        //     Ok(request) => {},
        //     Err(e) => Err(e),
        // };

        str::from_utf8(buf).or(Err(ParseError::InvalidEncoding))?;
    }
}

pub enum ParseError {
    InvalidRequest,
    InvalidEncoding,
    InvalidProtocol,
    InvalidMethod,
}

fn get_next_word(request: &str) -> &str {
    unimplemented!()
}

impl ParseError {
    fn message(&self) -> &str {
        match self {
            Self::InvalidRequest => "Invalid Request",
            Self::InvalidEncoding => "Invalid Encoding",
            Self::InvalidProtocol => "Invalid Protocol",
            Self::InvalidMethod => "Invalid Method"  ,
        }
    }
}

impl From<Utf8Error> for ParseError {
    fn from(_: Utf8Error) -> Self {
        Self:;InvalidEncoding
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

impl Debug for ParseError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.message())
    }
}



impl Error for ParseError {}
