use crate::tcp_protocol::{Packet, ProtocolCommand, Requester};
use board::{error::PlacingShipsError, OwnBoard, Ship};
use error::ConnectionError;
use tokio::net::TcpStream;

mod board;
mod error;

pub struct Player<'a> {
    ships: Vec<Ship>,
    own_board: OwnBoard<'a>,
    requester: Requester,
}
impl<'a> Player<'a> {
    pub fn new(stream: TcpStream) -> Self {
        Self {
            ships: Vec::new(),
            own_board: OwnBoard::new(),
            requester: Requester::new(stream),
        }
    }

    pub fn set_up(&'a mut self) -> Result<(), PlacingShipsError> {
        self.own_board.place_ships(&self.ships)?;
        Ok(())
    }

    pub async fn connect(&mut self) -> Result<(), ConnectionError> {
        let _res = self
            .requester
            .send_request(
                Packet::new(ProtocolCommand::Connect, "secret")
                    .expect("creating packet failed unexpectedly"),
            )
            .await?;
        Ok(())
    }
}
