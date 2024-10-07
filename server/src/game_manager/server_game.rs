use std::sync::{Arc, Mutex};

use super::server_player::ServerPlayer;

#[derive(Debug)]
pub struct ServerGame {
    player1: Arc<Mutex<ServerPlayer>>,
    player2: Option<Arc<Mutex<ServerPlayer>>>,
}
impl ServerGame {
    pub fn new(player: Arc<Mutex<ServerPlayer>>) -> Self {
        ServerGame {
            player1: player,
            player2: None,
        }
    }

    pub fn has_empty_slot(&self) -> bool {
        self.player2.is_none()
    }

    pub fn add_opponent(&mut self, player: Arc<Mutex<ServerPlayer>>) {
        self.player2 = Some(player);
    }
}