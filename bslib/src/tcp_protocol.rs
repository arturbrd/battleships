use std::io::Read;

use error::{PacketReaderError, RequestError};
use tokio::io::{self, AsyncBufReadExt, AsyncWriteExt, ReadHalf};
use tokio::io::{BufReader, WriteHalf};
use tokio::net::TcpStream;

pub mod error;

pub const PACKET_HEADER: &str = "#bs";
pub const PACKET_END: &str = "\n#end\n";

trait PacketBody {
    fn to_string(&self) -> String;
}
struct ConnectBody {
    nick: String,
}
impl PacketBody for ConnectBody {
    fn to_string(&self) -> String {
        String::from("test body")
    }
}

#[derive(PartialEq, Eq, Debug)]
pub enum ProtocolCommand {
    UnknownCmd,
    Connect,
    ConnectResp,
}
impl ProtocolCommand {
    pub fn get_str(&self) -> Option<&str> {
        match self {
            Self::UnknownCmd => None,
            Self::Connect => Some("connect"),
            Self::ConnectResp => Some("connect_resp"),
        }
    }

    pub fn from_cmd(cmd: &str) -> Option<ProtocolCommand> {
        match cmd {
            "connect" => Some(Self::Connect),
            "connect_resp" => Some(Self::ConnectResp),
            _ => None,
        }
    }
}

trait PacketState {}
struct NotReady;
impl PacketState for NotReady {}
struct Ready;
impl PacketState for Ready {}

#[derive(Debug)]
pub struct Packet<S: PacketState, BodyType: PacketBody> {
    command: ProtocolCommand,
    body: Option<String>,
    _phantom_body_type: std::marker::PhantomData<BodyType>,
    _phantom_s: std::marker::PhantomData<S>,
}
impl<S: PacketState, BodyType: PacketBody> Packet<S, BodyType> {
    pub fn new(cmd: ProtocolCommand) -> Option<Packet<NotReady, BodyType>> {
        if cmd == ProtocolCommand::UnknownCmd {
            None
        } else {
            Some(Packet::<NotReady, BodyType> {
                command: cmd,
                body: None,
                _phantom_s: std::marker::PhantomData,
                _phantom_body_type: std::marker::PhantomData,
            })
        }
    }

    pub fn as_bytes(&self) -> Box<[u8]> {
        let req = PACKET_HEADER.to_string()
            + " "
            + self.command.get_str().expect("couldn't get command str")
            + "\n"
            + self.body.as_ref().expect("this shouldn't break")
            + PACKET_END;
        let req = req.into_bytes();
        req.into_boxed_slice()
    }

    pub fn get_cmd(&self) -> &ProtocolCommand {
        &self.command
    }
}
impl<BodyType: PacketBody> Packet<NotReady, BodyType> {
    pub fn load_body(&mut self, body: BodyType) -> Packet<Ready, BodyType> {
        Packet {
            body: Some(body.to_string()),
            command: self.command,
            _phantom_s: std::marker::PhantomData,
            _phantom_body_type: std::marker::PhantomData,
        }
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
        Self {
            packet_reader,
            write_half,
        }
    }

    pub async fn send_request(&mut self, request: Packet) -> Result<Response, RequestError> {
        self.write_half.write_all(&request.as_bytes()).await?;
        self.write_half.flush().await?;
        let response = self.packet_reader.read_packet().await?;
        println!("{:#?}", response);
        match response {
            Some(_) => Ok(Response {}),
            None => Err(RequestError::new(String::from("Response not received"))),
        }
    }
}

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
            return Ok(None);
        };
        let (header, cmd) = buf
            .trim()
            .split_once(' ')
            .unwrap_or_else(|| panic!("failed to split a request: {:}", buf));

        let body = self.read_body().await?;
        if header != PACKET_HEADER {
            return Err(PacketReaderError::new(String::from("Wrong packet header")));
        }

        let cmd = ProtocolCommand::from_cmd(cmd);
        let packet = match cmd {
            Some(cmd) => Ok(Some(Packet::new(cmd, &body).expect("it shouldn't panic"))),
            None => Ok(Some(
                Packet::new(ProtocolCommand::UnknownCmd, &body).expect("it shouldn't panic"),
            )),
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
