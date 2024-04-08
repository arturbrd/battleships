pub mod player;
pub mod tcpProtocol;

use player::Player;

#[derive(Default)]
pub struct Game<'a> {
    player_one: Option<Player<'a>>,
    player_two: Option<Player<'a>>
}
impl <'a> Game<'a> {
    pub fn new() -> Self {
        Game::default()
    }
}