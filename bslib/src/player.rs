mod board;

use board::{OwnBoard, PlacingShipsError, Ship, ShipType};
use core::fmt::Display;
use tokio::io;
use tokio::net::TcpStream;

use crate::tcp_protocol::{self, ProtocolCommand, Packet, Requester};

#[derive(Debug, Clone)]
pub struct ConnectionError {
    msg: String,
}
impl Display for ConnectionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ConnectionError: {}", self.msg)
    }
}
impl From<io::Error> for ConnectionError {
    fn from(value: io::Error) -> Self {
        Self {
            msg: format!("{value:}"),
        }
    }
}
impl<T: tcp_protocol::ProtocolError> From<T> for ConnectionError {
    fn from(value: T) -> Self {
        Self {
            msg: format!("{value:}"),
        }
    }
}

pub struct Player<'a> {
    ships: Vec<Ship>,
    own_board: OwnBoard<'a>,
    requester: Requester
}
impl<'a> Player<'a> {
    pub fn new(stream: TcpStream) -> Self {
        Self {
            ships: Vec::new(),
            own_board: OwnBoard::new(),
            requester: Requester::new(stream)
        }
    }

    pub fn set_up(&'a mut self) -> Result<(), PlacingShipsError> {
        self.own_board.place_ships(&self.ships)?;
        Ok(())
    }

    pub async fn connect(&mut self) -> Result<(), ConnectionError> {
        let _res = self.requester.
            send_request(Packet::new(ProtocolCommand::Connect, "secret")
                .expect("creating packet failed unexpectedly"))
            .await?;
        Ok(())
    }
}
// impl<'a> Default for Player<'a> {
//     fn default() -> Self {
//         Player {
//             ships: vec![
//                 Ship::new(ShipType::Carrier),
//                 Ship::new(ShipType::Battleship),
//                 Ship::new(ShipType::Cruiser),
//                 Ship::new(ShipType::Submarine),
//                 Ship::new(ShipType::Destroyer),
//             ],
//             own_board: OwnBoard::new(),
//         }
//     }
// }
