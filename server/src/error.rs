use crate::handlers::error::HandlersModError;
use core::fmt::Display;
use std::sync::TryLockError;
use bslib::tcp_protocol::error::PacketError;
use tokio::io;
use tokio::sync::mpsc::error::SendError;

#[derive(Debug, Clone)]
pub struct HandlingError {
    msg: String,
}
impl HandlingError {
    pub fn new(msg: &str) -> Self {
        Self {
            msg: String::from(msg)
        }
    }
}
impl Display for HandlingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "HandlingError: {}", self.msg)
    }
}
impl<E: HandlersModError> From<E> for HandlingError {
    fn from(value: E) -> Self {
        Self {
            msg: format!("{value:}"),
        }
    }
}
impl<T> From<SendError<T>> for HandlingError {
    fn from(value: SendError<T>) -> Self {
        Self {
            msg: format!("{value:}"),
        }
    }
}
impl<T> From<TryLockError<T>> for HandlingError {
    fn from(value: TryLockError<T>) -> Self {
        Self {
            msg: format!("{value:}"),
        }
    }
}
impl From<PacketError> for HandlingError {
    fn from(value: PacketError) -> Self {
        Self {
            msg: format!("{value:}"),
        }
    }
}
impl From<io::Error> for HandlingError {
    fn from(value: io::Error) -> Self {
        Self {
            msg: format!("{value:}"),
        }
    }
}
impl From<tokio::task::JoinError> for HandlingError {
    fn from(value: tokio::task::JoinError) -> Self {
        Self {
            msg: format!("{value:}"),
        }
    }
}
impl From<bslib::tcp_protocol::error::PacketReaderError> for HandlingError {
    fn from(value: bslib::tcp_protocol::error::PacketReaderError) -> Self {
        Self {
            msg: format!("{value:}"),
        }
    }
}
impl std::error::Error for HandlingError {}
