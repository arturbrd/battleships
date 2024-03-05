struct Board<'a> {
    board: [[Tile<'a>; 10]; 10]
}
impl <'a> Board<'a> {
    fn new() -> Board<'a> {
        Board {
            board: [[Tile::new(); 10]; 10]
        }
    }
}

#[derive(Clone, Copy)]
struct Tile<'a> {
    shot: bool,
    ship: Option<&'a Ship>
}
impl <'a> Tile<'a> {
    fn new() -> Tile<'a> {
        Tile { shot: false, ship: None }
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
    destroyed: bool
}
impl Ship {
    fn new(ship_type: ShipType) -> Ship {
        let size = get_ship_size(&ship_type);
        Ship { ship_type, size, destroyed: false }
    }
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
