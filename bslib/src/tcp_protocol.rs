use error::{PacketReaderError, RequestError};
use tokio::io::{self, AsyncBufReadExt, AsyncWriteExt, ReadHalf};
use tokio::io::{BufReader, WriteHalf};
use tokio::net::TcpStream;

use self::error::PacketError;

pub mod error;

pub const PACKET_HEADER: &str = "#bs";
pub const PACKET_END: &str = "\n#end\n";

trait PacketBody: std::fmt::Debug + std::marker::Send {
    fn to_string(&self) -> String;
}

#[derive(Debug)]
struct TestBody {
    nick: String,
}
impl TestBody {
    pub fn new(nick: String) -> Self {
        Self { nick }
    }
}
impl PacketBody for TestBody {
    fn to_string(&self) -> String {
        String::from("test body")
    }
}

#[derive(Debug)]
pub struct ConnectBody {
    nick: String,
}
impl ConnectBody {
    pub fn new(nick: String) -> Self {
        Self { nick }
    }
}
impl PacketBody for ConnectBody {
    fn to_string(&self) -> String {
        String::from("test body")
    }
}

#[derive(Debug)]
pub struct ConnectRespBody {
    nick: String,
}
impl ConnectRespBody {
    pub fn new(nick: String) -> Self {
        Self { nick }
    }
}
impl PacketBody for ConnectRespBody {
    fn to_string(&self) -> String {
        String::from("test body")
    }
}

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
pub enum PacketBodyType {
    Test(Box<TestBody>),
    Connect(Box<ConnectBody>),
    ConnectResp(Box<ConnectRespBody>),
}
impl PacketBodyType {
    pub fn get_cmd(&self) -> ProtocolCommand {
        match self {
            Self::Test(_) => ProtocolCommand::Test,
            Self::Connect(_) => ProtocolCommand::Connect,
            Self::ConnectResp(_) => ProtocolCommand::ConnectResp,
        }
    }

    pub fn as_trait_obj(self) -> Box<dyn PacketBody> {
        match self {
            Self::Test(body) => body as Box<dyn PacketBody>,
            Self::Connect(body) => body as Box<dyn PacketBody>,
            Self::ConnectResp(body) => body as Box<dyn PacketBody>,
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
    body: Option<Box<dyn PacketBody>>,
    _phantom: std::marker::PhantomData<S>,
}
impl<S: BodyState> Packet<S> {
    pub fn get_cmd(&self) -> &ProtocolCommand {
        &self.command
    }
}
impl Packet<Ready> {
    pub fn as_bytes(&self) -> Box<[u8]> {
        if let Some(body) = &self.body {
            let req = PACKET_HEADER.to_string()
                + " "
                + self.command.get_str().expect("couldn't get command str")
                + "\n"
                + body.to_string().as_str()
                + PACKET_END;
            let req = req.into_bytes();
            req.into_boxed_slice()
        } else {
            panic!("This shouldn't happen for a Packet<Ready, _>");
        }
    }
}
impl Packet<NotReady> {
    pub fn load_body(self, body: PacketBodyType) -> Result<Packet<Ready>, PacketError> {
        let body_type = body.get_cmd();
        if body_type == self.command {
            Ok(Packet {
                command: self.command,
                body: Some(body.as_trait_obj()),
                _phantom: std::marker::PhantomData,
            })
        } else {
            Err(PacketError::new(String::from("Wrong body type")))
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

    pub async fn read_packet(&mut self) -> Result<Option<Packet<Ready>>, PacketReaderError> {
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
            Some(cmd) => Ok(Some(match cmd {
                ProtocolCommand::Test => {
                    let body = Box::new(TestBody::new(String::from("test body")));
                    Packet::new(cmd).load_body(PacketBodyType::Test(body))?
                }
                ProtocolCommand::Connect => {
                    let body = Box::new(ConnectBody::new(String::from("test body")));
                    Packet::new(cmd).load_body(PacketBodyType::Connect(body))?
                }
                ProtocolCommand::ConnectResp => {
                    let body = Box::new(ConnectRespBody::new(String::from("test body")));
                    Packet::new(cmd).load_body(PacketBodyType::ConnectResp(body))?
                }
            })),
            None => Err(PacketReaderError::new(String::from("Wrong command name"))),
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
