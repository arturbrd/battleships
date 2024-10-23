use error::{PacketReaderError, RequestError};
use serde::{Deserialize, Serialize};
use tokio::io::{self, AsyncBufReadExt, AsyncWriteExt, ReadHalf};
use tokio::io::{BufReader, WriteHalf};
use tokio::net::TcpStream;

use self::error::PacketError;

pub mod error;

pub const PACKET_HEADER: &str = "#bs";
pub const PACKET_END: &str = "\n#end\n";

pub trait Jsonable: Serialize + for <'a> Deserialize<'a> {
    fn from_json(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str::<Self>(json)
    }

    fn to_string(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct TestBody {
}
impl Jsonable for TestBody {}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConnectBody {
    nick: String,
}
impl ConnectBody {
    pub fn new(nick: String) -> Self {
        Self { nick }
    }

    fn get_nick(&self) -> &str {
        &self.nick
    }
}
impl Jsonable for ConnectBody {}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConnectRespBody {
    opponent: bool,
    game_id: u32
}
impl ConnectRespBody {
    pub fn new(opponent: bool, game_id: u32) -> Self {
        Self { opponent, game_id }
    }
}
impl Jsonable for ConnectRespBody {}

#[derive(PartialEq, Eq, Debug)]
pub enum ProtocolCommand {
    Test,
    Connect,
    ConnectResp,
}
impl ProtocolCommand {
    pub fn get_str(&self) -> Option<&str> {
        match self {
            Self::Connect => Some("connect"),
            Self::ConnectResp => Some("connect_resp"),
            Self::Test => Some("test"),
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

#[derive(Debug)]
pub enum PacketBody {
    Test(Box<TestBody>),
    Connect(Box<ConnectBody>),
    ConnectResp(Box<ConnectRespBody>),
}
impl PacketBody {
    pub fn get_cmd(&self) -> ProtocolCommand {
        match self {
            Self::Test(_) => ProtocolCommand::Test,
            Self::Connect(_) => ProtocolCommand::Connect,
            Self::ConnectResp(_) => ProtocolCommand::ConnectResp,
        }
    }

   pub fn to_string(&self) -> Result<String, serde_json::Error> {
        match self {
            Self::Test(body) => body.to_string(),
            Self::Connect(body) => body.to_string(),
            Self::ConnectResp(body) => body.to_string(),
        }
    }

    pub fn get_nick(&self) -> Result<&str, PacketError> {
        match self {
            Self::Test(_) => Err(PacketError::new("No such field on this type of body")),
            Self::Connect(body) => Ok(body.get_nick()),
            Self::ConnectResp(_) => Err(PacketError::new("No such field on this type of body"))
        }
    }
}

pub trait BodyState {}

#[derive(Debug)]
pub struct NotReady;
impl BodyState for NotReady {}

#[derive(Debug)]
pub struct Ready;
impl BodyState for Ready {}

#[derive(Debug)]
pub struct Packet<S: BodyState> {
    command: ProtocolCommand,
    body: Option<PacketBody>,
    _phantom: std::marker::PhantomData<S>,
}
impl<S: BodyState> Packet<S> {
    pub fn get_cmd(&self) -> &ProtocolCommand {
        &self.command
    }
}
impl Packet<Ready> {
    pub fn as_bytes(&self) -> Result<Box<[u8]>, serde_json::Error> {
        if let Some(body) = &self.body {
            let req = PACKET_HEADER.to_string()
                + " "
                + self.command.get_str().expect("couldn't get command str")
                + "\n"
                + body.to_string()?.as_str()
                + PACKET_END;
                println!("packet as string: {:#?}", req);
            let req = req.into_bytes();
            Ok(req.into_boxed_slice())
        } else {
            panic!("This shouldn't happen for a Packet<Ready, _>");
        }
    }

    pub fn get_body(&self) -> Result<&PacketBody, PacketError> {
        match &self.body {
            Some(body) => Ok(body),
            None => Err(PacketError::new("There is no body"))
        }
    }
}
impl Packet<NotReady> {
    pub fn load_body(self, body: PacketBody) -> Result<Packet<Ready>, PacketError> {
        let body_type = body.get_cmd();
        if body_type == self.command {
            Ok(Packet {
                command: self.command,
                body: Some(body),
                _phantom: std::marker::PhantomData,
            })
        } else {
            Err(PacketError::new("Wrong body type"))
        }
    }

    pub fn new(cmd: ProtocolCommand) -> Packet<NotReady> {
        Packet::<NotReady> {
            command: cmd,
            body: None,
            _phantom: std::marker::PhantomData,
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

    pub async fn send_request(&mut self, request: Packet<Ready>) -> Result<Response, RequestError> {
        println!("request: {:#?}", request);
        self.write_half.write_all(&request.as_bytes()?).await?;
        pause();
        self.write_half.flush().await?;
        let response = self.packet_reader.read_packet().await?;
        println!("{:#?}", response);
        match response {
            Some(_) => Ok(Response {}),
            None => Err(RequestError::new(String::from("Response not received"))),
        }
    }
}

fn pause() {
    dbg!("Pausing! Press enter to continue...");

    let mut buffer = String::new();

    std::io::stdin()
        .read_line(&mut buffer)
        .expect("Failed to read line");
}

pub struct PacketReader {
    reader: BufReader<ReadHalf<TcpStream>>,
}
impl PacketReader {
    pub fn new(reader: BufReader<ReadHalf<TcpStream>>) -> Self {
        Self { reader }
    }

    pub async fn read_packet(&mut self) -> Result<Option<Packet<Ready>>, PacketReaderError> {
        let mut buf = String::new();
        if self.reader.read_line(&mut buf).await? == 0 {
            return Ok(None);
        };
        let (header, cmd) = buf
            .trim()
            .split_once(' ')
            .unwrap_or_else(|| panic!("failed to split a request: {:}", buf));

        let raw_body = self.read_body().await?;
        println!("raw_body: {:#?}", raw_body);
        if header != PACKET_HEADER {
            return Err(PacketReaderError::new(String::from("Wrong packet header")));
        }

        let cmd = ProtocolCommand::from_cmd(cmd);
        let packet = match cmd {
            Some(cmd) => Ok(Some(match cmd {
                ProtocolCommand::Test => {
                    let body = Box::new(TestBody::from_json(&raw_body)?);
                    Packet::new(cmd).load_body(PacketBody::Test(body))?
                }
                ProtocolCommand::Connect => {
                    let body = Box::new(ConnectBody::from_json(&raw_body)?);
                    Packet::new(cmd).load_body(PacketBody::Connect(body))?
                }
                ProtocolCommand::ConnectResp => {
                    let body = Box::new(ConnectRespBody::from_json(&raw_body)?);
                    Packet::new(cmd).load_body(PacketBody::ConnectResp(body))?
                }
            })),
            None => Err(PacketReaderError::new(String::from("Wrong command name"))),
        };
        buf.clear();
        println!("packet: {:#?}", packet);
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
