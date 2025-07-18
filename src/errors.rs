use std::error::Error;
use std::fmt::{Display, Formatter, Result};

#[derive(Debug, PartialEq, Eq)]
pub enum InitError {
    EmptySecretWord,
    NonAlphabeticCharacters,
    NotEnoughGuesses,
}

impl Display for InitError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            InitError::EmptySecretWord => write!(f, "Secret word cannot be empty."),
            InitError::NonAlphabeticCharacters => {
                write!(f, "Secret word must contain only alphabetic characters.")
            }
            InitError::NotEnoughGuesses => write!(f, "Incorrect guess limit must be at least 1."),
        }
    }
}

impl Error for InitError {}
