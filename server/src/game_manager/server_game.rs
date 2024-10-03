use super::server_player::ServerPlayer;

pub struct ServerGame<'a> {
    player1: &'a ServerPlayer<'a>,
    player2: Option<&'a ServerPlayer<'a>>,
}
impl<'a> ServerGame<'a> {
    pub fn new(player: &'a ServerPlayer) -> Self {
        ServerGame {
            player1: player,
            player2: None,
        }
    }

    pub fn has_empty_slot(&self) -> bool {
        self.player2.is_none()
    }

    pub fn add_opponent(&mut self, player: &'a ServerPlayer) {
        self.player2 = Some(player);
    }
}
