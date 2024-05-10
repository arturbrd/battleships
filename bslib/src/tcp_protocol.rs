use std::fmt::Display;
use tokio::io::{BufReader, WriteHalf};
use tokio::io::{self, AsyncBufReadExt, AsyncWriteExt, ReadHalf};
use tokio::net::TcpStream;
use tokio::sync::mpsc::{Receiver, Sender};

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
impl std::convert::From<PacketReaderError> for RequestError {
    fn from(value: PacketReaderError) -> Self {
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

pub struct Requester {
    packet_reader: PacketReader,
    write_half: WriteHalf<TcpStream>,
}
impl Requester {
    pub fn new(stream: TcpStream) -> Self {
        let (read_half, write_half) = tokio::io::split(stream);
        let packet_reader = PacketReader::new(BufReader::new(read_half));
        Self { packet_reader, write_half }
    }

    pub async fn send_request(&mut self, request: Packet) -> Result<Response, RequestError> {
        self.write_half.write_all(&request.as_bytes()).await?;
        self.write_half.flush().await?;
        let response = self.packet_reader.read_packet().await?;
        println!("{:#?}", response);
        match response {
            Some(_) => {
                Ok(Response {})
            }
            None => {
                Err(RequestError {
                    msg: "Response not received".to_owned(),
                })
            }
        }
    }    
}

#[derive(Debug)]
pub struct PacketReaderError {
    msg: String,
}
impl Display for PacketReaderError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "PacketReaderError: {}", self.msg)
    }
}
impl std::convert::From<io::Error> for PacketReaderError {
    fn from(value: io::Error) -> Self {
        Self {
            msg: format!("{value:}"),
        }
    }
}
impl<T> std::convert::From<tokio::sync::mpsc::error::SendError<T>> for PacketReaderError {
    fn from(value: tokio::sync::mpsc::error::SendError<T>) -> Self {
        Self {
            msg: format!("{value:}"),
        }
    }
}
impl std::error::Error for PacketReaderError {}
impl ProtocolError for PacketReaderError {} 

pub struct PacketReader {
    reader: BufReader<ReadHalf<TcpStream>>,
}
impl PacketReader {
    pub fn new(reader: BufReader<ReadHalf<TcpStream>>) -> Self {
        Self { reader }
    }

    pub async fn read_packet(&mut self) -> Result<Option<Packet>, PacketReaderError> {
        let mut buf = String::new();
        if self.reader.read_line(&mut buf).await? == 0 {
            return Ok(None)
        };
        let (header, cmd) = buf
            .trim()
            .split_once(' ')
            .unwrap_or_else(|| panic!("failed to split a request: {:}", buf));

        let body = self.read_body().await?;
        if header != PACKET_HEADER {
            return Err(PacketReaderError { msg: String::from("Wrong packet header") });
        }

        let cmd = ProtocolCommand::from_cmd(cmd);
        let packet = match cmd {
            Some(cmd) => {
                Ok(Some(Packet::new(cmd, &body).expect("it shouldn't panic")))
            }
            None => {
                Ok(Some(Packet::new(ProtocolCommand::UnknownCmd, &body).expect("it shouldn't panic")))
            }
        };
        buf.clear();
        packet

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
