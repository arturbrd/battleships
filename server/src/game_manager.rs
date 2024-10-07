use std::sync::{Arc, Mutex};

use server_game::ServerGame;
use server_player::ServerPlayer;

mod server_game;
pub mod server_player;

#[derive(Debug, Default)]
pub struct GameManager {
    games: Vec<ServerGame>,
}
impl GameManager {
    fn create_game(&mut self, player: Arc<Mutex<ServerPlayer>>) {
        self.games.push(ServerGame::new(player));
    }

    pub fn assign_player(&mut self, player: Arc<Mutex<ServerPlayer>>) {
        for game in &mut self.games {
            if game.has_empty_slot() {
                game.add_opponent(player);
                println!("assigned a player to a game, game_manager be like: {:#?}", self);
                return;
            }
        }
        self.create_game(player);
        println!("assigned a player to a game, game_manager be like: {:#?}", self);
    }
}
