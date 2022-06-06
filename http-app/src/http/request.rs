use super::method::Method;
use std::{convert::TryFrom, error::Error, fmt::{ Display, Debug, Formatter, Result as FmtResult}};
use std::str;
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
        //
        //
        let (method,request) =get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (path ,request) =get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (protocol, _) =get_next_word(request).ok_or(ParseError::InvalidRequest)?;

        if protocol != "HTTP/1.1" {
            return Err(ParseError::InvalidProtocol);
        }

        let method:Method = method.parse()?;

        let mut query_string = None;

        if let Some(i) = path.find("?") {
            query_string = Some(&path[i + 1..]);
            path = &path[..i];
        }

        Ok(Self {
            path: path.to_string(),
            query_string: query_string.to_,
            method

        })





        str::from_utf8(buf).or(Err(ParseError::InvalidEncoding))?;

        unimplemented!()
    }
}

pub enum ParseError {
    InvalidRequest,
    InvalidEncoding,
    InvalidProtocol,
    InvalidMethod,
}

fn get_next_word(request: &str) -> Option<(&str, &str)>{
    for (i, c ) in request.chars().enumerate(){
        if c == ' ' || c == '\r'{
            return Some((&request[..i], &request[i + 1..]))
        }
    }
    None
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

impl From<MethodError> for ParseError {
    fn from(_: MethodError) -> Self {
        Self::InvalidMethod
    }
}
impl From<Utf8Error> for ParseError {
    fn from(_: Utf8Error) -> Self {
        Self::InvalidEncoding
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
