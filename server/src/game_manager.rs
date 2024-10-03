use bslib::game::Game;

pub struct GameManager {
    games: Vec<Game>,
}
impl GameManager {
    pub fn new() -> Self {
        GameManager { games: Vec::new() }
    }

    pub fn create_game(&mut self) {
        self.games.push(Game::new());
    }

    pub fn find_empty_slot(&self) -> Option<&Game> {
        for game in self.games {
            if game.has_empty_slot() {
                return Some(&game);
            }
        }
        None
    }
}
