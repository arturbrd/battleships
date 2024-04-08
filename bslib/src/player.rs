mod board;

use board::{Ship, ShipType, OwnBoard, PlacingShipsError};
use core::fmt::Display;
use tokio::net::TcpStream;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[derive(Debug, Clone)]
pub struct ConnectionError {
    msg: &'static str
}
impl Display for ConnectionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ConnectionError: {}", self.msg)
    }
}

pub struct Player<'a> {
    ships: Vec<Ship>,
    own_board: OwnBoard<'a>
}
impl <'a> Player<'a> {
    pub fn new() -> Self {
        Player::default()
    }

    pub fn set_up(&'a mut self) -> Result<(), PlacingShipsError> {
        self.own_board.place_ships(&self.ships)?;
        Ok(())
    }

    pub async fn connect(&'a self, stream: &mut TcpStream) -> Result<(), ConnectionError> {
        let conn_cmd = "#battleships connect".as_bytes();
        if stream.write_all(conn_cmd).await.is_err() {
            return Err(ConnectionError { msg: "Connecting to a game failed" });
        }
        let mut buf = String::new();
        if stream.read_to_string(&mut buf).await.is_err() {
            return Err(ConnectionError { msg: "Connecting to a game failed" });
        };
        println!("co: {:}", buf);
        if buf == "#battleship connect_ack" {
            Ok(())
        } else {
            Err(ConnectionError { msg: "Connecting to a game failed" })
        }
    }
}
impl <'a> Default for Player<'a> {
    fn default() -> Self {
        Player { ships: vec![Ship::new(ShipType::Carrier), Ship::new(ShipType::Battleship), Ship::new(ShipType::Cruiser), Ship::new(ShipType::Submarine), Ship::new(ShipType::Destroyer)], own_board: OwnBoard::new()}
    }
}