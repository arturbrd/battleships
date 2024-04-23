use std::fmt::Display;
use tokio::io::AsyncWriteExt;
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
impl std::error::Error for RequestError {}

pub enum ProtocolCommand {
    TEST(),
}

pub struct Request {
    command: ProtocolCommand,
}
impl Request {
    pub fn as_bytes(&self) -> &[u8] {
        b"#bs test\n#end"
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

    pub async fn send_request(self, request: Request) -> Response {
        self.stream.write_all(request.as_bytes()).await;
        Response {}
    }
}
