use bslib::tcp_protocol::{Packet, PacketReader, ProtocolCommand, Ready};
use config::{Config, Environment};
use dotenv::dotenv;
use error::HandlingError;
use game_manager::server_player::ServerPlayer;
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

    loop {
        let (stream, _) = listener
            .accept()
            .await
            .expect("failed to establish a connection");

        tokio::spawn(async move {
            if let Err(e) = handle_connection(stream).await {
                println!("Error: {e}");
            } else {
                println!("Handled perfectly");
            }
        });
    }
}

async fn handle_connection(stream: TcpStream) -> Result<(), HandlingError> {
    println!("Handling connection");
    let (read_half, mut write_half) = tokio::io::split(stream);

    let (tx, mut rx) = mpsc::channel(128);

    let player = ServerPlayer::new();

    let listener = tokio::spawn(async move {
        let packet_reader = PacketReader::new(io::BufReader::new(read_half));
        listen_stream(packet_reader, tx).await
    });

    while let Some(packet) = rx.recv().await {
        println!("{:#?}", packet);
        decode_handler(packet, &mut write_half, &player).await?;
    }
    let _ = listener.await??;
    Ok(())
}

async fn listen_stream(
    mut packet_reader: PacketReader,
    tx: Sender<Packet<Ready>>,
) -> Result<(), HandlingError> {
    while let Some(packet) = packet_reader.read_packet().await? {
        println!("sending packet to handle_connect");
        tx.send(packet).await?;
    }
    Ok(())
}

async fn decode_handler(
    packet: Packet<Ready>,
    write_half: &mut WriteHalf<TcpStream>,
    player: &ServerPlayer
) -> Result<(), HandlingError> {
    match packet.get_cmd() {
        ProtocolCommand::Connect => handlers::handle_connect_cmd(write_half, player).await?,
        _ => (),
    }
    println!("handler has finished");
    Ok(())
}
