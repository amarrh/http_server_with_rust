use super::method::{Method, MethodError}; ////super keyword used for finding modules one level up, ovdje definišemo tako da ne moramo kasnije
use super::QueryString;
use std::convert::TryFrom;
use std::error::Error;
use std::fmt::{Display, Result as FmtResult, Formatter, Debug};
use std::str::{self, Utf8Error};

#[derive(Debug)]
pub struct Request<'buf>{
    path: &'buf str,
    query_string: Option<QueryString<'buf>>, //Option enum wrapping the String value, value can be None, won't throw NullPointerException
    method: Method, //Method enum, da nije gore definisano morao bi ovdje method: super::method::Method;
}

impl<'buf> Request<'buf>{
    pub fn path(&self) -> &str{
        &self.path
    }

    pub fn query_string(&self) -> Option<&QueryString>{
        self.query_string.as_ref()
    }

    pub fn method(&self) -> &Method{
        &self.method
    }
}

impl<'buf> TryFrom<&'buf [u8]> for Request<'buf>{
    type Error = ParseErrror;

    // GET /search?name=abc&sort1 HTTP/1.1
    fn try_from(buf: &'buf [u8]) -> Result<Request<'buf>, Self::Error>{
        let request = str::from_utf8(buf)?; //zamjena za match, #39
        /*match get_next_word(request){
            Some((method,request))=>{},
            None => return Err(ParseErrror::InvalidRequest),
        }*/ //Iduća linija koda isto kao ovo gore
        let (method, request) = get_next_word(request).ok_or(ParseErrror::InvalidRequest)?;
        let (mut path, request) = get_next_word(request).ok_or(ParseErrror::InvalidRequest)?;
        let (protocol, _) = get_next_word(request).ok_or(ParseErrror::InvalidRequest)?;

        if protocol != "HTTP/1.1" {
            return Err(ParseErrror::InvalidProtocol)
        }
        let method: Method = method.parse()?;
        let mut query_string = None;
        
        if let Some(i) = path.find('?'){
            query_string = Some(QueryString::from(&path[i + 1..]));
            path = &path[..i];
        }
       Ok(Self{
           path, 
           query_string, 
           method,
       })
    }
}

fn get_next_word(request: &str) -> Option<(&str, &str)>{
    for (i, c) in request.chars().enumerate(){
        if c == ' ' || c == '\r'{
            return Some((&request[..i], &request[i + 1..]));
        }
    }
    None
}

pub enum ParseErrror{
    InvalidRequest,
    InvalidEncoding,
    InvalidProtocol,
    InvalidMethod,
}

impl ParseErrror{
    fn message(&self) -> &str{
        match self{
            Self::InvalidRequest => "Invalid Request",
            Self::InvalidEncoding => "Invalid Encoding",
            Self::InvalidProtocol => "Invalid Protocol",
            Self::InvalidMethod => "Invalid Method"
        }
    }
}

impl From<MethodError> for ParseErrror{
    fn from(_: MethodError) -> Self{
        Self::InvalidMethod
    }
}

impl From<Utf8Error> for ParseErrror{
    fn from(_: Utf8Error) -> Self{
        Self::InvalidEncoding
    }
}

impl Display for ParseErrror{
    fn fmt(&self, f: &mut Formatter) -> FmtResult{
        write!(f, "{}", self.message())
    }
}

impl Debug for ParseErrror{
    fn fmt(&self, f: &mut Formatter) -> FmtResult{
        write!(f, "{}", self.message())
    }
}

impl Error for ParseErrror{}