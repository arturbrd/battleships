use std::fmt::Display;
use tokio::io::{BufReader, WriteHalf};
use tokio::io::{self, AsyncBufReadExt, AsyncWriteExt, ReadHalf};
use tokio::net::TcpStream;
use tokio::sync::mpsc::Sender;

pub const PACKET_HEADER: &str = "#bs";
pub const PACKET_END: &str = "\n#end\n";

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

#[derive(PartialEq, Eq, Debug)]
pub enum ProtocolCommand {
    UnknownCmd,
    Connect,
    ConnectResp
}
impl ProtocolCommand {
    pub fn get_str(&self) -> Option<&str> {
        match self {
            Self::UnknownCmd => None,
            Self::Connect => Some("connect"),
            Self::ConnectResp => Some("connect_resp")
        }
    }

    pub fn from_cmd(cmd: &str) -> Option<ProtocolCommand> {
        match cmd {
            "connect" => Some(Self::Connect),
            "connect_resp" => Some(Self::ConnectResp),
            _ => None
        }
    }
}

#[derive(Debug)]
pub struct Packet {
    command: ProtocolCommand,
    body: String
}
impl Packet {
    pub fn new(cmd: ProtocolCommand, body: &str) -> Option<Packet> {
        if cmd == ProtocolCommand::UnknownCmd {
            None
        } else {
            Some(Packet { command: cmd, body: String::from(body) })
        }
    }

    pub fn as_bytes(&self) -> Box<[u8]> {
        let req = PACKET_HEADER.to_string() + " " + self.command.get_str().expect("couldn't get command str") + "\n" + &self.body + PACKET_END;
        let req = req.into_bytes();
        req.into_boxed_slice()
    }

    pub fn get_cmd(&self) -> &ProtocolCommand {
        &self.command
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

    pub async fn send_request(&mut self, request: Packet) -> Result<Response, RequestError> {
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

#[derive(Debug)]
pub struct ReaderError {
    msg: String,
}
impl Display for ReaderError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ReaderError: {}", self.msg)
    }
}
impl std::convert::From<io::Error> for ReaderError {
    fn from(value: io::Error) -> Self {
        Self {
            msg: format!("{value:}"),
        }
    }
}
impl<T> std::convert::From<tokio::sync::mpsc::error::SendError<T>> for ReaderError {
    fn from(value: tokio::sync::mpsc::error::SendError<T>) -> Self {
        Self {
            msg: format!("{value:}"),
        }
    }
}
impl std::error::Error for ReaderError {}
impl ProtocolError for ReaderError {} 

pub struct PacketReader {
    reader: BufReader<ReadHalf<TcpStream>>,
    tx: Sender<Option<Packet>>
}
impl PacketReader {
    pub fn new(reader: BufReader<ReadHalf<TcpStream>>, tx: Sender<Option<Packet>>) -> Self {
        Self { reader, tx }
    }

    pub async fn listen_stream(&mut self) -> Result<(), ReaderError> {
        let mut buf = String::new();
        while self.reader.read_line(&mut buf).await? != 0 {
            println!("header: {}", buf);
            let (header, cmd) = buf
                .trim()
                .split_once(' ')
                .unwrap_or_else(|| panic!("failed to split a request: {:}", buf));

            if header != PACKET_HEADER {
                self.tx.send(None).await?;
            }
            let body = self.read_body().await?;
            
            let cmd = ProtocolCommand::from_cmd(cmd);
            match cmd {
                Some(cmd) => {
                    self.tx.send(Packet::new(cmd, &body)).await;
                }
                None => {
                    self.tx.send(Packet::new(ProtocolCommand::UnknownCmd, &body)).await;
                },
            }
            buf.clear();

        }
        Ok(())
    }

    pub async fn read_body(&mut self) -> Result<String, io::Error> {
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
