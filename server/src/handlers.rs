use std::sync::Arc;
use std::sync::Mutex;

use error::ConnectError;
use tokio::io::AsyncWriteExt;
use tokio::io::WriteHalf;
use tokio::net::TcpStream;

use crate::game_manager::server_player::ServerPlayer;
use crate::game_manager::GameManager;

pub mod error;

pub async fn handle_unknown() -> Result<(), ConnectError> {
    Ok(())
}

pub async fn handle_connect_cmd<'a: 'b, 'b: 'c, 'c>(stream: &mut WriteHalf<TcpStream>, player: Arc<Mutex<ServerPlayer>>, game_manager: &'b Arc<Mutex<GameManager>>) -> Result<(), ConnectError> {
    println!("handle_connect_cmd");
    {
        let mut game_manager = game_manager.try_lock()?;
        game_manager.assign_player(player);
    }
    stream
        .write_all("#bs connect_resp\n#end\n".as_bytes())
        .await?;
    stream.flush().await?;
    println!("handle_connect_cmd finished");
    Ok(())
}
