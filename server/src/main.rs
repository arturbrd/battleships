use config::{Config, Environment};
use core::fmt::Display;
use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use tokio::io::{self, AsyncBufReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

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

async fn handle_connection(mut stream: TcpStream) -> Result<(), HandlingError> {
    println!("Handling connection");
    let mut buf = String::new();
    let mut reader = io::BufReader::new(&mut stream);
    reader.read_line(&mut buf).await?;
    reader.consume(buf.len());
    println!("{}", buf);
    let (header, cmd) = buf
        .trim()
        .split_once(' ')
        .unwrap_or_else(|| panic!("failed to split a request: {:}", buf));

    if header != "#battleships" {
        stream
            .write_all("#battleships connect_rej".as_bytes())
            .await?;
        return Err(HandlingError {
            msg: "wrong header".to_owned(),
        });
    }
    match cmd {
        "connect" => {
            handlers::handle_connect_cmd(&mut stream).await?;
            Ok(())
        }
        _ => Err(HandlingError {
            msg: "no such command".to_owned(),
        }),
    }
}
