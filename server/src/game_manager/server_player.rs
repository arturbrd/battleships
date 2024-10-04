#[derive(Debug)]
pub struct ServerPlayer {
    nick: String
}
impl ServerPlayer {
    pub fn new() -> Self {
        ServerPlayer{ nick: String::new() }
    }

    pub fn set_nick(&mut self, nick: &str) {
        self.nick = String::from(nick);
    }
}