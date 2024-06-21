use bslib::player::Player;
use tokio::net::TcpStream;

#[tokio::main]
async fn main() {
    let stream = TcpStream::connect("127.0.0.1:8000").await.unwrap();
    println!("Hello, world!");
    let mut player = Player::new(stream);
    player.connect().await.expect("failed to connect to a game");
    println!("Connecting again");
    player.connect().await.expect("failed to connect to a game");

    player.set_up().expect("failed to set up a board");
}
