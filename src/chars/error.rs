use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug, PartialEq, Eq)]
pub enum CharError {
    NonAlphabetic,
}

impl Display for CharError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            CharError::NonAlphabetic => {
                write!(f, "Only alphabetic characters are allowed.")
            }
        }
    }
}

impl Error for CharError {}
