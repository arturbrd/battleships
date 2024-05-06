mod board;

use board::{OwnBoard, PlacingShipsError, Ship, ShipType};
use core::fmt::Display;
use tokio::io::{self, AsyncBufReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

use crate::tcp_protocol::{self, ProtocolCommand, Request, Requester};

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
}
impl<'a> Player<'a> {
    pub fn new() -> Self {
        Player::default()
    }

    pub fn set_up(&'a mut self) -> Result<(), PlacingShipsError> {
        self.own_board.place_ships(&self.ships)?;
        Ok(())
    }

    pub async fn connect(&'a self, stream: &mut TcpStream) -> Result<(), ConnectionError> {
        let mut requester = Requester::new(stream);
        let response = requester.send_request(Request::new(ProtocolCommand::CONNECT)).await?;
        Ok(())

     
    }
}
impl<'a> Default for Player<'a> {
    fn default() -> Self {
        Player {
            ships: vec![
                Ship::new(ShipType::Carrier),
                Ship::new(ShipType::Battleship),
                Ship::new(ShipType::Cruiser),
                Ship::new(ShipType::Submarine),
                Ship::new(ShipType::Destroyer),
            ],
            own_board: OwnBoard::new(),
        }
    }
}
