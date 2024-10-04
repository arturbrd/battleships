use bslib::tcp_protocol::error::ProtocolError;
use core::fmt::Display;

#[derive(Debug, Clone)]
pub struct ConnectionError {
    msg: String,
}
impl Display for ConnectionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ConnectionError: {}", self.msg)
    }
}
// impl From<io::Error> for ConnectionError {
//     fn from(value: io::Error) -> Self {
//         Self {
//             msg: format!("{value:}"),
//         }
//     }
// }
impl<T: ProtocolError> From<T> for ConnectionError {
    fn from(value: T) -> Self {
        Self {
            msg: format!("{value:}"),
        }
    }
}
