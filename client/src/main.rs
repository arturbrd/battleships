use std::io::prelude::*;
use std::net::TcpStream;
use std::thread;
use std::time::Duration;
use bslib::player::Player;

fn main() {
    let mut stream = TcpStream::connect("127.0.0.1:8000").unwrap();
    let sample = "#battleships\ntest\n".as_bytes();
    stream.write_all(sample).unwrap();
    thread::sleep(Duration::from_millis(1000));
    println!("Hello, world!");
    stream.write_all(sample).unwrap();

    let mut player = Player::new();
    player.set_up().expect("failed to set up a board");

    // let mut my_board = OwnBoard::new();
    // let battleship = Ship::new(ShipType::Battleship);
    // let carrier = Ship::new(ShipType::Carrier);
    // let submarine = Ship::new(ShipType::Submarine);
    // let destroyer = Ship::new(ShipType::Destroyer);
    // let cruiser = Ship::new(ShipType::Cruiser);
    // let ships = vec![carrier, battleship, cruiser, submarine, destroyer];
    // println!("{:?}", my_board.place_ships(&ships).expect("place ships nie działa"));
    // println!("{}", my_board);

}
