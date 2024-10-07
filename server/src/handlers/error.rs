use core::fmt::Display;
use std::sync::TryLockError;
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
impl<T> From<TryLockError<T>> for ConnectError {
    fn from(value: TryLockError<T>) -> Self {
        Self {
            msg: format!("{value:}"),
        }
    }
}
