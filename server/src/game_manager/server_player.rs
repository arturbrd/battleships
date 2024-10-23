#[derive(Debug, Default)]
pub struct ServerPlayer {
    nick: String,
    own_board: ServerOwnBoard,
    enemy_board: ServerEnemyBoard
}
impl ServerPlayer {
    pub fn set_nick(&mut self, nick: &str) {
        self.nick = String::from(nick);
    }
}