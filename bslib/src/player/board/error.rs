use core::fmt::Display;
use std::num::ParseIntError;

#[derive(Debug, Clone)]
pub struct PlacingShipsError {
    msg: String,
}
impl PlacingShipsError {
    pub fn new(msg: String) -> Self {
        Self { msg }
    }
}
impl Display for PlacingShipsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "PlacingShipsError: {}", self.msg)
    }
}

#[derive(Debug, Clone)]
pub struct UserInputError {
    msg: String,
}
impl UserInputError {
    pub fn new(msg: String) -> Self {
        Self { msg }
    }
}
impl Display for UserInputError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "UserInputError: {}", self.msg)
    }
}
impl std::convert::From<ParseIntError> for UserInputError {
    fn from(value: ParseIntError) -> Self {
        Self {
            msg: format!("{value:}"),
        }
    }
}
