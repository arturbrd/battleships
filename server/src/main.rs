use dotenv::dotenv;
use config::{Config, Environment};
use serde::{Serialize, Deserialize};
use std::io::BufReader;
use tokio::net::{TcpListener, TcpStream};


// mod handlers;

#[derive(Serialize, Deserialize)]
pub struct ServerConfig {
    pub server_addr: String
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let config = Config::builder()
        .add_source(Environment::default().separator("__"))
        .build()
        .unwrap();

    let config: ServerConfig = config.try_deserialize().unwrap();

    let listener = TcpListener::bind(config.server_addr).await.expect("failed to create a listener");

    loop {
        let (stream, addr) = listener.accept().await.expect("failed to establish a connection");

        tokio::spawn(async move {
            handle_connection(stream).await;
        });
    }
}

async fn handle_connection(stream: TcpStream) {
    let buffer = BufReader::new(&mut stream);
}
