mod player;

use player::Player;

pub struct Game<'a> {
    player_one: Option<Player<'a>>,
    player_two: Option<Player<'a>>
}
impl <'a> Game<'a> {
    pub fn new() -> Game<'a> {
        Game {player_one: None, player_two: None}
    }
}