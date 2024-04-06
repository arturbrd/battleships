use dotenv::dotenv;
use config::{Config, Environment};
use serde::{Serialize, Deserialize};
use tokio::io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader, self};
use tokio::net::{TcpListener, TcpStream};
use core::fmt::Display;


mod handlers;

#[derive(Serialize, Deserialize)]
pub struct ServerConfig {
    pub server_addr: String
}

#[derive(Debug, Clone)]
pub struct HandlingError {
    msg: &'static str
}
impl Display for HandlingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "PlacingShipsError: {}", self.msg)
    }
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
            if let Err(e) = handle_connection(stream).await {
                println!("Error: {e}");
            } else {
                println!("Handled perfectly");
            }
        });
    }
}

async fn handle_connection(mut stream: TcpStream) -> Result<(), HandlingError> {
    println!("Handling connection");
    let mut buf = String::new();
    stream.read_to_string(&mut buf).await.unwrap();
    println!("{}", buf);
    let (header, cmd) = buf.split_once(' ').unwrap_or_else(|| panic!("failed to split a request: {:}", buf));
    if header != "#battleships" {
        if stream.write_all("#battleships connect_rej".as_bytes()).await.is_err() {
            return Err(HandlingError { msg: "IO error"});
        } else {
            return Err(HandlingError { msg: "wrong header"});
        }
    }
    match cmd {
        "connect" => {
            if handlers::handle_connect_cmd(&mut stream).await.is_err() {
                Err(HandlingError { msg: "connect_cmd failed" })
            } else {
                Ok(())
            }
        }
        _ => Err(HandlingError { msg: "no such command"})
    }
}
