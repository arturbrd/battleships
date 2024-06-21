use core::fmt::Display;
use tokio::io;

pub trait HandlersModError: std::error::Error {}

#[derive(Debug, Clone)]
pub struct ConnectError {
    msg: String,
}
impl Display for ConnectError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ConnectError: {}", self.msg)
    }
}
impl std::error::Error for ConnectError {}
impl HandlersModError for ConnectError {}
impl From<io::Error> for ConnectError {
    fn from(value: io::Error) -> Self {
        Self {
            msg: format!("{value:}"),
        }
    }
}
