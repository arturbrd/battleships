use core::fmt::Display;
use bslib::tcp_protocol::PacketReader;
use tokio::io::{self, WriteHalf};
use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;

pub trait HandlersModError: std::error::Error {}

#[derive(Debug, Clone)]
pub struct ConnectError {
    msg: String,
}
impl Display for ConnectError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ConnectError: {}", self.msg)
    }
}
impl std::error::Error for ConnectError {}
impl HandlersModError for ConnectError {}
impl From<io::Error> for ConnectError {
    fn from(value: io::Error) -> Self {
        Self {
            msg: format!("{value:}"),
        }
    }
}

pub async fn handle_unknown(packet_manager: &mut PacketReader) -> Result<(), ConnectError> {
    Ok(())
}

pub async fn handle_connect_cmd(stream: &mut WriteHalf<TcpStream>) -> Result<(), ConnectError> {
    stream
        .write_all("#bs connect_resp\n#end\n".as_bytes())
        .await?;
    stream.flush().await?;
    Ok(())
}


