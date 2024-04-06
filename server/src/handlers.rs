use core::fmt::Display;

use tokio::{io::AsyncWriteExt, net::TcpStream};

#[derive(Debug, Clone)]
pub struct ConnectError {
    msg: &'static str
}
impl Display for ConnectError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "PlacingShipsError: {}", self.msg)
    }
}

pub async fn handle_connect_cmd(stream: &mut TcpStream) -> Result<(), ConnectError> {
    if stream.write_all("#battleship connect_ack".as_bytes()).await.is_err() {
        return Err(ConnectError { msg: "failed to connect a player to a game"});
    }
    Ok(())
}