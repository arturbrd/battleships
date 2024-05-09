use config::{Config, Environment};
use tokio::sync::mpsc;
use core::fmt::Display;
use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use tokio::io::{self, WriteHalf};
use tokio::net::{TcpListener, TcpStream};
use bslib::tcp_protocol::{Packet, PacketReader, ProtocolCommand};

mod handlers;

#[derive(Serialize, Deserialize)]
pub struct ServerConfig {
    pub server_addr: String,
}

#[derive(Debug, Clone)]
pub struct HandlingError {
    msg: String,
}
impl Display for HandlingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "HandlingError: {}", self.msg)
    }
}
impl<E: handlers::HandlersModError> From<E> for HandlingError {
    fn from(value: E) -> Self {
        Self {
            msg: format!("{value:}"),
        }
    }
}
impl From<io::Error> for HandlingError {
    fn from(value: io::Error) -> Self {
        Self {
            msg: format!("{value:}"),
        }
    }
}
impl From<tokio::task::JoinError> for HandlingError {
    fn from(value: tokio::task::JoinError) -> Self {
        Self {
            msg: format!("{value:}")
        }
    }
}
impl From<bslib::tcp_protocol::ReaderError> for HandlingError {
    fn from(value: bslib::tcp_protocol::ReaderError) -> Self {
        Self {
            msg: format!("{value:}")
        }
    }
}
impl std::error::Error for HandlingError {}

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

    let listener = tokio::spawn(async move {
        let mut packet_reader = PacketReader::new(io::BufReader::new(read_half), tx);
        packet_reader.listen_stream().await
    });

    while let Some(packet) = rx.recv().await {
        if let Some(packet) = packet {
            println!("{:#?}", packet);
            let result = decode_handler(packet, &mut write_half).await?;            
        } else {
            println!("Wrong header");
        } 
    }
    let _ = listener.await??;
    Ok(())
}

async fn decode_handler(packet: Packet, write_half: &mut WriteHalf<TcpStream>) -> Result<(), HandlingError> {
    match packet.get_cmd() {
        ProtocolCommand::Connect => {
            handlers::handle_connect_cmd(write_half).await?
        },
        _ => ()
    }
    Ok(())
}

