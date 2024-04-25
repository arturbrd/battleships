mod board;

use board::{OwnBoard, PlacingShipsError, Ship, ShipType};
use core::fmt::Display;
use tokio::io::{self, AsyncBufReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

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
        let conn_cmd = "#battleships connect\n".as_bytes();
        stream.write_all(conn_cmd).await?;
        let mut reader = io::BufReader::new(stream);
        let mut buf = String::new();
        reader.read_line(&mut buf).await?;
        reader.consume(buf.len());
        buf = buf.trim().to_owned();
        println!("co: {:#?}", buf);
        if buf == "#battleship connect_ack" {
            Ok(())
        } else {
            Err(ConnectionError {
                msg: "Connecting to a game failed".to_owned(),
            })
        }
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
