use server_game::ServerGame;
use server_player::ServerPlayer;

mod server_game;
pub mod server_player;

pub struct GameManager {
    games: Vec<ServerGame>,
}
impl GameManager {
    fn new() -> Self {
        GameManager { games: Vec::new() }
    }

    fn create_game(&mut self, player: &'a ServerPlayer) {
        self.games.push(ServerGame::new(player));
    }

    fn find_empty_slot(&self) -> Option<&ServerGame> {
        for game in self.games {
            if game.has_empty_slot() {
                return Some(&game);
            }
        }
        None
    }

    pub fn assign_player(&self, player: &ServerPlayer) {
        match self.find_empty_slot() {
            Some(game)  => {
                game.add_opponent(player);
            },
            None => {
                self.create_game(player);
            }
        }
    }
}
