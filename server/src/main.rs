use dotenv::dotenv;
use config::{Config, Environment};
use serde::{Serialize, Deserialize};
use tokio::io::{AsyncBufReadExt, AsyncReadExt, BufReader};
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
        let (stream, _) = listener.accept().await.expect("failed to establish a connection");

        tokio::spawn(async move {
            handle_connection(stream).await;
        });
    }
}

async fn handle_connection(mut stream: TcpStream) {
    let mut buf = Vec::new();
    stream.read_to_end(&mut buf).await.unwrap();
    println!("{}", String::from_utf8(buf).unwrap());
}
