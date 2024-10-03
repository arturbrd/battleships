use server_game::ServerGame;
use server_player::ServerPlayer;

mod server_game;
pub mod server_player;

pub struct GameManager<'a> {
    games: Vec<ServerGame<'a>>,
}
impl<'a> GameManager<'a> {
    fn new() -> Self {
        Self { games: Vec::new() }
    }

    fn create_game(&mut self, player: &'a ServerPlayer) {
        self.games.push(ServerGame::new(player));
    }

    fn find_empty_slot(&mut self) -> Option<&'a mut ServerGame> {
        for game in &mut self.games {
            if game.has_empty_slot() {
                return Some(game);
            }
        }
        None
    }

    pub fn assign_player(&'a mut self, player: &'a ServerPlayer) {
        let game_slot = self.find_empty_slot();
        match game_slot {
            Some(game)  => {
                game.add_opponent(player);
            },
            None => {
                self.create_game(player);
            }
        }
    }
}
