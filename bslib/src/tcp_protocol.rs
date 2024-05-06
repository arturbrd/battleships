use std::fmt::Display;
use tokio::io::{self, AsyncWriteExt, AsyncBufReadExt};
use tokio::net::TcpStream;

pub trait ProtocolError: std::error::Error {}

#[derive(Debug)]
pub struct RequestError {
    msg: String,
}
impl Display for RequestError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "RequestError: {}", self.msg)
    }
}
impl std::convert::From<io::Error> for RequestError {
    fn from(value: io::Error) -> Self {
        Self {
            msg: format!("{value:}"),
        }
    }
}
impl std::convert::From<ResponseError> for RequestError {
    fn from(value: ResponseError) -> Self {
        Self {
            msg: format!("{value:}")
        }
    }
}
impl std::error::Error for RequestError {}
impl ProtocolError for RequestError {} 

#[derive(Debug)]
pub struct ResponseError {
    msg: String,
}
impl Display for ResponseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ResponseError: {}", self.msg)
    }
}
impl std::convert::From<io::Error> for ResponseError {
    fn from(value: io::Error) -> Self {
        Self {
            msg: format!("{value:}"),
        }
    }
}
impl std::error::Error for ResponseError {}
impl ProtocolError for ResponseError {} 

pub enum ProtocolCommand {
    TEST,
    CONNECT
}
impl ProtocolCommand {
    pub fn get_cmd(&self) -> String {
        match self {
            Self::TEST => "test".to_string(),
            Self::CONNECT => "connect".to_string()
        }
    }
}

pub struct Request {
    command: ProtocolCommand,
}
impl Request {
    pub fn new(cmd: ProtocolCommand) -> Self {
        Request { command: cmd }
    }
    pub fn as_bytes(&self) -> Box<[u8]> {
        let req = "#bs ".to_string() + self.command.get_cmd().as_str() + "\n#end";
        let req = req.into_bytes();
        req.into_boxed_slice()
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

    pub async fn send_request(&mut self, request: Request) -> Result<Response, RequestError> {
        let _ = self.stream.write_all(&request.as_bytes()).await?;
        self.stream.flush().await?;
        self.read_response().await?;
        Ok(Response {})
    }

    async fn read_response(&mut self) -> Result<Response, ResponseError> {
        let mut reader = io::BufReader::new(&mut self.stream);
        let mut buf = String::new();
        reader.read_line(&mut buf).await?;
        reader.consume(buf.len());
        buf = buf.trim().to_owned();
        println!("response: {:#?}", buf);
        if buf == "#bs connect_ack" {
            Ok(Response {})
        } else {
            Err(ResponseError {
                msg: "Connecting to a game failed".to_owned(),
            })
        }
    }
}

