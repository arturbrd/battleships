use std::fmt::Display;
use tokio::io::{AsyncWriteExt, self};
use tokio::net::TcpStream;

#[derive(Debug)]
pub struct RequestError {
    msg: String
}
impl Display for RequestError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "RequestError: {}", self.msg)
    }
}
impl std::convert::From<io::Error> for RequestError {
    fn from(value: io::Error) -> Self {
        Self { msg: format!("{value:}")} 
    }
}
impl std::error::Error for RequestError {}

pub enum ProtocolCommand {
    TEST(),
}

pub struct Request {
    command: ProtocolCommand,
}
impl Request {
    pub fn as_bytes(&self) -> Box<[u8]> {
        Box::new(*b"#bs test\n#end")
    }
}

pub struct Response {}

pub struct Requester<'a> {
    stream: &'a mut TcpStream,
}
impl<'a> Requester<'a> {
    pub fn new(stream: &'a mut TcpStream) -> Self {
        Self { stream }
    }

    pub async fn send_request(self, request: Request) -> Result<Response, RequestError> {
        let _ = self.stream.write_all(&request.as_bytes()).await?;
        self.stream.flush().await?;
        Ok(Response {})
    }
}
