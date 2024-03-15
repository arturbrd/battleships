mod board;

use board::{Ship, ShipType, OwnBoard, PlacingShipsError};

pub struct Player<'a> {
    ships: Vec<Ship>,
    own_board: OwnBoard<'a>
}
impl <'a> Player<'a> {
    pub fn new() -> Player<'a> {
        Player { ships: vec![Ship::new(ShipType::Battleship), Ship::new(ShipType::Carrier), Ship::new(ShipType::Cruiser), Ship::new(ShipType::Submarine), Ship::new(ShipType::Destroyer)], own_board: OwnBoard::new()}
    }

    pub fn set_up(&'a mut self) -> Result<(), PlacingShipsError> {
        self.own_board.place_ships(&self.ships)?;
        Ok(())
    }
}