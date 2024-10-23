use std::sync::Arc;
use std::sync::Mutex;

use bslib::tcp_protocol;
use bslib::tcp_protocol::ConnectRespBody;
use bslib::tcp_protocol::Packet;
use bslib::tcp_protocol::PacketBody;
use bslib::tcp_protocol::ProtocolCommand;
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
    let (opponent, game_id) = {
        let mut game_manager = game_manager.try_lock()?;
        game_manager.assign_player(player)
    };
    let body = Box::new(ConnectRespBody::new(opponent, game_id));
    let response = Packet::new(ProtocolCommand::ConnectResp).load_body(PacketBody::ConnectResp(body))?;
    stream
        .write_all(response.as_bytes()?.as_ref())
        .await?;
    stream.flush().await?;
    println!("handle_connect_cmd finished");
    Ok(())
}
