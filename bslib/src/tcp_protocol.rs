use std::fmt::Display;
use tokio::io::BufReader;
use tokio::io::{self, AsyncBufReadExt, AsyncWriteExt, ReadHalf};
use tokio::net::TcpStream;

const PACKET_HEADER: &str = "#bs";
const PACKET_END: &str = "\n#end\n";

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
    Connect,
    ConnectResp{ status: String }
}
impl ProtocolCommand {
    pub fn get_cmd(&self) -> &str {
        match self {
            Self::Connect => "connect",
            Self::ConnectResp{ .. } => "connect_resp"
        }
    }
}

pub struct Request {
    command: ProtocolCommand,
    body: String
}
impl Request {
    pub fn new(cmd: ProtocolCommand, body: &str) -> Self {
        Request { command: cmd, body: String::from(body) }
    }
    pub fn as_bytes(&self) -> Box<[u8]> {
        let req = PACKET_HEADER.to_string() + " " + self.command.get_cmd() + "\n" + &self.body + PACKET_END;
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
        self.stream.write_all(&request.as_bytes()).await?;
        self.stream.flush().await?;
        self.read_response().await?;
        Ok(Response {})
    }

    async fn read_response(&mut self) -> Result<Response, ResponseError> {
        let mut reader = io::BufReader::new(&mut self.stream);
        let mut buf = String::new();
        reader.read_line(&mut buf).await?;
        reader.consume(buf.len());
        buf = buf.trim().to_string();
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

pub struct PacketReader<'a> {
    reader: &'a mut BufReader<ReadHalf<TcpStream>>,
}
impl<'a> PacketReader<'a> {
    pub fn new(reader: &'a mut BufReader<ReadHalf<TcpStream>>) -> Self {
        Self { reader }
    }

    pub async fn read_packet(self) -> Result<String, io::Error> {
        let mut buf = String::new();
        let mut packet = String::new();
        while self.reader.read_line(&mut buf).await? != 0 {
            if buf == "#end\n" {
                buf.clear();
                break;
            }
            packet.push_str(&buf);
            buf.clear()
        }       
        Ok(packet)
    }
}
