use bslib::player::Player;
use tokio::net::TcpStream;

#[tokio::main]
async fn main() {
    let mut stream = TcpStream::connect("127.0.0.1:8000").await.unwrap();
    // let sample = "#battleships\ntest\n".as_bytes();
    // stream.write_all(sample).unwrap();
    // thread::sleep(Duration::from_millis(1000));
    println!("Hello, world!");
    // stream.write_all(sample).unwrap();
    // stream.shutdown(std::net::Shutdown::Both).unwrap();

    let mut player = Player::new(stream);
    player
        .connect()
        .await
        .expect("failed to connect to a game");
    println!("Connecting again");
    player
        .connect()
        .await
        .expect("failed to connect to a game");
 
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
