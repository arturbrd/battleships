use crate::tcp_protocol::{ConnectBody, Packet, PacketBodyType, ProtocolCommand, Requester};
use board::{error::PlacingShipsError, OwnBoard, Ship, ShipType};
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
            ships: vec![
                Ship::new(ShipType::Carrier),
                Ship::new(ShipType::Battleship),
                Ship::new(ShipType::Cruiser),
                Ship::new(ShipType::Submarine),
                Ship::new(ShipType::Destroyer),
            ],
            own_board: OwnBoard::new(),
            requester: Requester::new(stream),
        }
    }

    pub fn set_up(&'a mut self) -> Result<(), PlacingShipsError> {
        self.own_board.place_ships(&self.ships)?;
        Ok(())
    }

    pub async fn connect(&mut self) -> Result<(), ConnectionError> {
        let body =
            PacketBodyType::Connect(Box::new(ConnectBody::new(String::from("connect body"))));
        let _res = self
            .requester
            .send_request(Packet::new(ProtocolCommand::Connect).load_body(body)?)
            .await?;
        Ok(())
    }
}
