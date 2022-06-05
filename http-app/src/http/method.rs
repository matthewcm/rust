use std::str::FromStr;

use super::ParseError;



pub enum Method {
    GET,
    POST,
    PATCH,
    DELETE,
    PUT,
    HEAD,
    CONNECT,
    OPTIONS,
    TRACE
}

impl FromStr for Method {

    type Err = MethodError;


    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "GET"=> Ok(Self::GET),
            "POST"=> Ok(Self::POST),
            "PATCH"=> Ok(Self::PATCH),
            "DELETE"=> Ok(Self::DELETE),
            "PUT" => Ok(Self::PUT),
            "HEAD" => Ok(Self::HEAD),
            "CONNECT" => Ok(Self::CONNECT),
            "OPTIONS" => Ok(Self::OPTIONS),
            "TRACE" => Ok(Self::TRACE),
            _ => Err(MethodError)
        }
    }
}

pub struct MethodError;
