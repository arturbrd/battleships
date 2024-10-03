use crate::player::Player;

pub struct Game {
    player1: &Player,
    player2: Option<&Player>,
}
impl Game {
    pub fn new(player: &Player) -> Self {
        Game {
            player1: player,
            player2: None,
        }
    }

    pub fn has_empty_slot(&self) -> bool {
        self.player2.is_none()
    }
}
