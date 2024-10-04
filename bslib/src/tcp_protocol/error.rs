use std::fmt::Display;
use tokio::io;

pub trait ProtocolError: std::error::Error {}

#[derive(Debug)]
pub struct RequestError {
    msg: String,
}
impl RequestError {
    pub fn new(msg: String) -> Self {
        Self { msg }
    }
}
impl Display for RequestError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "RequestError: {}", self.msg)
    }
}
impl std::convert::From<io::Error> for RequestError {
    fn from(value: io::Error) -> Self {
        Self {
            msg: format!("{value:}"),
        }
    }
}
impl std::convert::From<ResponseError> for RequestError {
    fn from(value: ResponseError) -> Self {
        Self {
            msg: format!("{value:}"),
        }
    }
}
impl std::convert::From<PacketReaderError> for RequestError {
    fn from(value: PacketReaderError) -> Self {
        Self {
            msg: format!("{value:}"),
        }
    }
}
impl std::error::Error for RequestError {}
impl ProtocolError for RequestError {}

#[derive(Debug)]
pub struct ResponseError {
    msg: String,
}
impl ResponseError {
    pub fn new(msg: String) -> Self {
        Self { msg }
    }
}
impl Display for ResponseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ResponseError: {}", self.msg)
    }
}
impl std::convert::From<io::Error> for ResponseError {
    fn from(value: io::Error) -> Self {
        Self {
            msg: format!("{value:}"),
        }
    }
}
impl std::error::Error for ResponseError {}
impl ProtocolError for ResponseError {}

#[derive(Debug)]
pub struct PacketReaderError {
    msg: String,
}
impl PacketReaderError {
    pub fn new(msg: String) -> Self {
        Self { msg }
    }
}
impl Display for PacketReaderError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "PacketReaderError: {}", self.msg)
    }
}
impl std::convert::From<io::Error> for PacketReaderError {
    fn from(value: io::Error) -> Self {
        Self {
            msg: format!("{value:}"),
        }
    }
}
impl<T> std::convert::From<tokio::sync::mpsc::error::SendError<T>> for PacketReaderError {
    fn from(value: tokio::sync::mpsc::error::SendError<T>) -> Self {
        Self {
            msg: format!("{value:}"),
        }
    }
}
impl std::convert::From<PacketError> for PacketReaderError {
    fn from(value: PacketError) -> Self {
        Self {
            msg: format!("{value:}"),
        }
    }
}
impl std::error::Error for PacketReaderError {}
impl ProtocolError for PacketReaderError {}

#[derive(Debug)]
pub struct PacketError {
    msg: String,
}
impl PacketError {
    pub fn new(msg: &str) -> Self {
        Self { msg: String::from(msg) }
    }
}
impl Display for PacketError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "PacketError: {}", self.msg)
    }
}
impl std::convert::From<io::Error> for PacketError {
    fn from(value: io::Error) -> Self {
        Self {
            msg: format!("{value:}"),
        }
    }
}
impl<T> std::convert::From<tokio::sync::mpsc::error::SendError<T>> for PacketError {
    fn from(value: tokio::sync::mpsc::error::SendError<T>) -> Self {
        Self {
            msg: format!("{value:}"),
        }
    }
}
impl std::error::Error for PacketError {}
impl ProtocolError for PacketError {}
