use super::{
    method::{Method, MethodError},
    QueryString,
};
use std::{
    convert::TryFrom,
    fmt::{self, Debug, Display},
    str::{self, Utf8Error},
};

#[derive(Debug)]
pub struct Request<'buff> {
    path: &'buff str,
    query_string: Option<QueryString<'buff>>,
    method: Method,
}

impl<'buff> Request<'buff> {
    pub fn path(&self) -> &str {
        &self.path
    }

    pub fn method(&self) -> &Method {
        &self.method
    }

    pub fn query_string(&self) -> Option<&QueryString> {
        self.query_string.as_ref()
    }
}

impl<'buff> TryFrom<&'buff [u8]> for Request<'buff> {
    type Error = ParseError;

    // GET /auth/google?code=asdasdasdsada HTTP/1.1

    fn try_from(buff: &'buff [u8]) -> Result<Self, Self::Error> {
        let request = str::from_utf8(buff)?;

        let (method, request) = get_next_token(request).ok_or(ParseError::InvalidRequest)?;
        let (mut path, request) = get_next_token(request).ok_or(ParseError::InvalidRequest)?;
        let (protocol, _) = get_next_token(request).ok_or(ParseError::InvalidRequest)?;

        if protocol != "HTTP/1.1" {
            return Err(ParseError::InvalidProtocol);
        }

        let method: Method = method.parse()?;

        let mut query_string = None;

        if let Some(i) = path.find("?") {
            query_string = Some(QueryString::from(&path[i + i..]));
            path = &path[..i];
        }

        Ok(Self {
            path,
            query_string,
            method,
        })
    }
}

fn get_next_token(request: &str) -> Option<(&str, &str)> {
    for (i, c) in request.chars().enumerate() {
        if c == ' ' || c == '\r' {
            return Some((&request[..i], &request[i + 1..]));
        }
    }

    return None;
}

pub enum ParseError {
    InvalidRequest,
    InvalidEncoding,
    InvalidProtocol,
    InvalidMethod,
}

impl Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message())
    }
}

impl Debug for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message())
    }
}

impl From<Utf8Error> for ParseError {
    fn from(_: Utf8Error) -> Self {
        return Self::InvalidEncoding;
    }
}

impl From<MethodError> for ParseError {
    fn from(_: MethodError) -> Self {
        return Self::InvalidMethod;
    }
}

impl ParseError {
    fn message(&self) -> &str {
        match self {
            Self::InvalidRequest => "InvalidRequest",
            Self::InvalidEncoding => "InvalidEncoding",
            Self::InvalidProtocol => "InvalidProtocol",
            Self::InvalidMethod => "InvalidMethod",
        }
    }
}
