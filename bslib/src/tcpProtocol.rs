use std::{fmt::Display, sync::Mutex};

use tokio::{io::{BufReader, BufWriter}, net::TcpStream};

#[derive(Debug)]
pub struct RequestError {
    msg: String
}
impl Display for RequestError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "RequestError: {}", self.msg)
    }
}
impl std::error::Error for RequestError {}

pub struct Request {}

pub struct Response {}

pub struct TcpProtocol<'a> {
    stream: &'a mut TcpStream
}
impl<'a> TcpProtocol<'a> {
    pub fn new(stream: &'a mut TcpStream) -> Self {
        Self { stream }
    }
    pub async fn sendRequest(&self, request: Request) -> Result<Response, RequestError> {
        Ok(Response {})
    }
}