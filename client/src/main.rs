use std::io::stdin;

use client_player::ClientPlayer;
use tokio::net::TcpStream;

mod client_player;

#[tokio::main]
async fn main() {
    let stream = TcpStream::connect("127.0.0.1:8000").await.unwrap();
    println!("Hello, world!");
    let nick = get_nick();
    let mut player = ClientPlayer::new(stream);
    player.connect(nick).await.expect("failed to connect to a game");

    player.set_up().expect("failed to set up a board");
}

fn get_nick() -> String {
    loop {
        println!("Set your nick: ");
        let mut buf = String::new();
        if stdin().read_line(&mut buf).is_err() {
            continue;
        } else {
            return buf;
        }
    }
}
