#[derive(Debug, Default)]
pub struct ServerPlayer {
    nick: String
}
impl ServerPlayer {
    pub fn set_nick(&mut self, nick: &str) {
        self.nick = String::from(nick);
    }
}