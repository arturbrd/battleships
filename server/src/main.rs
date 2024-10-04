use std::sync::{Arc, Mutex};

use bslib::tcp_protocol::{Packet, PacketReader, ProtocolCommand, Ready};
use config::{Config, Environment};
use dotenv::dotenv;
use error::HandlingError;
use game_manager::server_player::ServerPlayer;
use game_manager::GameManager;
use serde::{Deserialize, Serialize};
use tokio::io::{self, WriteHalf};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::mpsc;
use tokio::sync::mpsc::Sender;

mod error;
pub mod game_manager;
pub mod handlers;

#[derive(Serialize, Deserialize)]
pub struct ServerConfig {
    pub server_addr: String,
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let config = Config::builder()
        .add_source(Environment::default().separator("__"))
        .build()
        .unwrap();

    let config: ServerConfig = config.try_deserialize().unwrap();
    let listener = TcpListener::bind(config.server_addr)
        .await
        .expect("failed to create a listener");


    let game_manager = Arc::new(Mutex::new(GameManager::new()));

    loop {
        let (stream, _) = listener
            .accept()
            .await
            .expect("failed to establish a connection");
        let game_manager_clone = game_manager.clone();
        tokio::spawn(async move {
            if let Err(e) = handle_connection(stream, game_manager_clone).await {
                println!("Error: {e}");
            } else {
                println!("Handled perfectly");
            }
        });
    }
}

async fn handle_connection(stream: TcpStream, game_manager: Arc<Mutex<GameManager>>) -> Result<(), HandlingError> {
    println!("Handling connection");
    let (read_half, mut write_half) = tokio::io::split(stream);

    let (tx, mut rx) = mpsc::channel(128);

    let player = Arc::new(Mutex::new(ServerPlayer::new()));

    let listener = tokio::spawn(async move {
        let packet_reader = PacketReader::new(io::BufReader::new(read_half));
        listen_stream(packet_reader, tx).await
    });

    while let Some(packet) = rx.recv().await {
        println!("{:#?}", packet);
        decode_handler(packet, &mut write_half, player.clone(), &game_manager).await?;
    }
    listener.await??;
    Ok(())
}

async fn listen_stream(
    mut packet_reader: PacketReader,
    tx: Sender<Packet<Ready>>,
) -> Result<(), HandlingError> {
    while let Some(packet) = packet_reader.read_packet().await? {
        println!("sending packet to handle_connection");
        tx.send(packet).await?;
    }
    Ok(())
}

async fn decode_handler<'a: 'b, 'b: 'c, 'c>(
    packet: Packet<Ready>,
    write_half: &mut WriteHalf<TcpStream>,
    player: Arc<Mutex<ServerPlayer>>,
    game_manager: &'b Arc<Mutex<GameManager>>
) -> Result<(), HandlingError> {
    println!("decode handler: {:#?}", packet.get_cmd());
    match packet.get_cmd() {
        ProtocolCommand::Connect => {
            let body = packet.get_body()?;
            let nick = body.get_nick()?;
            {
                let mut player = player.try_lock()?;
                player.set_nick(nick);
            }

            handlers::handle_connect_cmd(write_half, player, game_manager).await?
        },
        ProtocolCommand::Test => (),
        _ => (),
    }
    println!("handler has finished");
    Ok(())
}
