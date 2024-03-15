use std::{fmt::Display, io::stdin};

#[derive(Debug, Clone)]
struct PlacingShipsError {
    msg: &'static str
}
impl Display for PlacingShipsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "PlacingShipsError: {}", self.msg)
    }
}

#[derive(Debug, Clone)]
struct UserInputError {
    msg: &'static str
}
impl Display for UserInputError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "UserInputError: {}", self.msg)
    }
}

const COORDINATES_LETTERS: &str = "abcdefghij";

// player's board with their ships
struct OwnBoard<'a> {
    board: [[OwnTile<'a>; 10]; 10],
    ships_placed: bool
}
impl <'a> OwnBoard<'a> {
    // create new instance
    fn new() -> OwnBoard<'a> {
        OwnBoard {
            board: [[OwnTile::new(); 10]; 10],
            ships_placed: false
        }
    }
    // prompt user to place their ships
    fn place_ships(&mut self, ships: &'a Vec<Ship>) -> Result<(), PlacingShipsError> {
        if self.ships_placed {
            return Err(PlacingShipsError {msg: "Ships were already placed"});
        };
        for ship in ships {
            self.place_ship(ship)?
        }
        Ok(())
    }
    // method used by place_ships to place one ship
    fn place_ship(&mut self, ship: &'a Ship) -> Result<(), PlacingShipsError> {
        print!("\x1B[2J\x1B[1;1H");
        loop {
            println!("{}", self);
            println!("Place your {} ({} tiles long) - enter tiles coordinates like this >>a1-a3<<:", ship, ship.size);
            let mut buf = String::new();
            if let Err(e) = stdin().read_line(&mut buf) {
                print!("\x1B[2J\x1B[1;1H");
                println!("Couldn't read form stdin! - {} - Trying again...\n", e);
                continue;
            }
            let coordinates = Self::decode_ship_placing_input(buf.trim(), ship);
            match coordinates {
                Ok(coordinates) => {
                    if let Err(e) = self.place_on_tiles(&coordinates, ship) {
                        print!("\x1B[2J\x1B[1;1H");
                        println!("{} - trying again...\n", e);
                        continue;
                    }
                    break;
                },
                Err(e) => {
                    print!("\x1B[2J\x1B[1;1H");
                    println!("Couldn't convert to coordinates! - {} - Trying again...\n", e);
                    continue;
                }
            }
        }
        Ok(())
    }
    // method decoding user's ship placing input
    fn decode_ship_placing_input(input: &str, ship: &Ship) -> Result<Vec<[usize; 2]>, UserInputError> {
        if !input.contains('-') {
            return Err(UserInputError {msg: "Missing hyphen"});
        }
        let fields = input.split('-');
        let mut decoded_indexes = Vec::new();
        for field in fields {
            let chars = field.chars().collect::<Vec<_>>();
            if (chars.len() == 3 || chars.len() == 2) && COORDINATES_LETTERS.contains(chars[0]) && chars[1].is_ascii_digit() {
                if chars.len() == 3 && !chars[2].is_ascii_digit() {
                    println!("{:#?}", chars);
                    return Err(UserInputError {msg: "Wrong format"});
                }
                let (i, j) = chars.split_at(1);
                let i = COORDINATES_LETTERS.find(i).ok_or(UserInputError {msg: "Such letters are not allowed in coordinates"})?;
                let j: usize = j.iter().collect::<String>().parse().map_err(|_| UserInputError {msg: "Cannot convert to a number"})?;
                decoded_indexes.push([i,j-1]);
            } else {
                return Err(UserInputError {msg: "Wrong format"});
            }
        }
        let (changing_coord, unchanging_coord): (usize, usize) = if decoded_indexes[0][0] == decoded_indexes[1][0] {
            (1, 0)
        } else if decoded_indexes[0][1] == decoded_indexes[1][1] {
            (0, 1)
        } else {
            return Err(UserInputError {msg: "Coordinates are not in line"});
        };
        let (greater, lesser) = if decoded_indexes[0][changing_coord] >= decoded_indexes[1][changing_coord] {
            (decoded_indexes[0][changing_coord], decoded_indexes[1][changing_coord])
        } else {
            (decoded_indexes[1][changing_coord], decoded_indexes[0][changing_coord])
        };
        if greater - lesser == ship.size as usize - 1 {
            for i in 1..ship.size-1 {
                if unchanging_coord == 0 {
                    decoded_indexes.insert(i as usize, [decoded_indexes[0][unchanging_coord], lesser + i as usize])
                } else {
                    decoded_indexes.insert(i as usize, [lesser + i as usize, decoded_indexes[0][unchanging_coord]])
                }
            }
        } else {
            return Err(UserInputError {msg: "This range is either too long or too short for this ship"})
        }
        Ok(decoded_indexes)
    }

    fn place_on_tiles(&mut self, coordinates: &Vec<[usize; 2]>, ship: &'a Ship) -> Result<(), PlacingShipsError> {
        for [i, j] in coordinates {
            if self.board[*i][*j].ship.is_some() {
                return Err(PlacingShipsError {msg: "Tile is not empty"});
            }

            let top = if *i > 0 {i-1} else {0};
            let down = if *i < 9 {i+1} else {9};
            let left = if *j > 0 {j-1} else {0};
            let right = if *j < 9 {j+1} else {9};
            
            for k in top..down+1 {
                for l in left..right+1 {
                    if self.board[k][l].ship.is_some() {
                        return Err(PlacingShipsError {msg: "The tile is next to another ship"});
                    }
                }
            }
        }
        for [i, j] in coordinates {
            self.board[*i][*j].ship = Some(ship);
        }
        Ok(())
    }
}
impl Display for OwnBoard<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut string = String::from("  | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10|\n-------------------------------------------\n");
        let chars = COORDINATES_LETTERS.chars().collect::<Vec<_>>();
        for (i, row) in self.board.iter().enumerate() {
            string += chars[i].to_string().as_str();
            string += " |";
            for tile in row {
                string += format!("{}|", tile).as_str();
            }
            string += "\n-------------------------------------------\n";
        }
        write!(f, "{}", string)
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
impl Display for OwnTile<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.shot {
            true => {
                match self.ship {
                    None => write!(f, " * "),
                    Some(_) => write!(f, " X ")
                }
            },
            false => {
                match self.ship {
                    None => write!(f, "   "),
                    Some(_) => write!(f, " @ ")
                }
            }
        }
    }
}

#[derive(PartialEq, Eq)]
enum ShipType {
    Carrier,
    Battleship,
    Cruiser,
    Submarine,
    Destroyer
}

#[derive(PartialEq, Eq)]
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

    fn get_ship_name(ship_type: &ShipType) -> &'static str {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_placing() {
        let mut my_board = OwnBoard::new();
        let battleship = Ship::new(ShipType::Battleship);
        let carrier = Ship::new(ShipType::Carrier);
        let submarine = Ship::new(ShipType::Submarine);
        let destroyer = Ship::new(ShipType::Destroyer);
        let cruiser = Ship::new(ShipType::Cruiser);
        let ships = vec![carrier, battleship, cruiser, submarine, destroyer];
        println!("{:?}", my_board.place_ships(&ships).expect("place ships nie działa"));
        println!("{}", my_board);
    }
}
