use std::{fmt::Display, io::stdin};

struct PlacingShipsError;
struct UserInputError;

struct OwnBoard<'a> {
    board: [[OwnTile<'a>; 10]; 10],
    ships_placed: bool
}
impl <'a> OwnBoard<'a> {
    fn new() -> OwnBoard<'a> {
        OwnBoard {
            board: [[OwnTile::new(); 10]; 10],
            ships_placed: false
        }
    }
    fn place_ships(&mut self, ships: &Vec<Ship>) -> Result<(), PlacingShipsError> {
        if self.ships_placed {
            return Err(PlacingShipsError);
        };
        for ship in ships {
            self.place_ship(ship)?
        }
        Ok(())
    }
    fn place_ship(&mut self, ship: &Ship) -> Result<(), PlacingShipsError> {
        loop {
            println!("Place your {} ({} tiles long) - enter tiles coordinates like this >>a1-a3<<:", ship, ship.size);
            let mut buf = String::new();
            if let Err(e) = stdin().read_line(&mut buf) {
                println!("Couldn't read form stdin! Trying again...");
                continue;
            }

        }
    }
    fn decode_ship_placing_input(input: &String) -> Result<(), UserInputError> {
        if input.len() != 5 {
            return Err(UserInputError);
        }
        let chars: Vec<_> = input.chars().collect();
        // TODO
        Ok(())
    }
}

#[derive(Clone, Copy)]
struct OwnTile<'a> {
    shot: bool,
    ship: Option<&'a Ship>
}
impl <'a> OwnTile<'a> {
    fn new() -> OwnTile<'a> {
        OwnTile { shot: false, ship: None }
    }
}

enum ShipType {
    Carrier,
    Battleship,
    Cruiser,
    Submarine,
    Destroyer
}

struct Ship {
    ship_type: ShipType,
    size: u8,
    name: &'static str,
    destroyed: bool
}
impl Ship {
    fn new(ship_type: ShipType) -> Ship {
        let size = Ship::get_ship_size(&ship_type);
        let name = Ship::get_ship_name(&ship_type);
        Ship { ship_type, size, name, destroyed: false }
    }
    fn get_ship_size(ship_type: &ShipType) -> u8 {
        const CARRIER_SIZE: u8 = 5;
        const BATTLESHIP_SIZE: u8 = 4;
        const CRUISER_SIZE: u8 = 3;
        const SUBMARINE_SIZE: u8 = 3;
        const DESTROYER_SIZE: u8 = 2;
    
        match ship_type {
            ShipType::Carrier => CARRIER_SIZE,
            ShipType::Battleship => BATTLESHIP_SIZE,
            ShipType::Cruiser => CRUISER_SIZE,
            ShipType::Submarine => SUBMARINE_SIZE,
            ShipType::Destroyer => DESTROYER_SIZE
        }
    }
    fn get_ship_name(ship_type: &ShipType) -> &str {
        const CARRIER_NAME: &str = "carrier";
        const BATTLESHIP_NAME: &str = "battleship";
        const CRUISER_NAME: &str = "cruiser";
        const SUBMARINE_NAME: &str = "submarine";
        const DESTROYER_NAME: &str = "destroyer";
    
        match ship_type {
            ShipType::Carrier => CARRIER_NAME,
            ShipType::Battleship => BATTLESHIP_NAME,
            ShipType::Cruiser => CRUISER_NAME,
            ShipType::Submarine => SUBMARINE_NAME,
            ShipType::Destroyer => DESTROYER_NAME
        }
    }
    
}
impl Display for Ship {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

