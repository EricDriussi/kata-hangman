use std::error::Error;
use std::fmt::{Display, Formatter, Result};

// TODO: Too much crap in this file, should be split up
// Maybe create modules?

#[derive(Debug, PartialEq, Eq)]
// TODO: Does this InitError make sense? Should it be simpler or just use error::Error?
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

impl From<AllowedFailuresError> for InitError {
    fn from(error: AllowedFailuresError) -> Self {
        match error {
            AllowedFailuresError::NotEnoughGuesses => InitError::NotEnoughGuesses,
        }
    }
}

impl From<SecretWordError> for InitError {
    fn from(error: SecretWordError) -> Self {
        match error {
            SecretWordError::EmptySecretWord => InitError::EmptySecretWord,
            SecretWordError::NonAlphabeticCharacters => InitError::NonAlphabeticCharacters,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum AllowedFailuresError {
    // TODO: NotEnoughFailures?
    NotEnoughGuesses,
}

impl Display for AllowedFailuresError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            AllowedFailuresError::NotEnoughGuesses => {
                write!(f, "Incorrect guess limit must be at least 1.")
            }
        }
    }
}

impl Error for AllowedFailuresError {}

#[derive(Debug, PartialEq, Eq)]
pub enum SecretWordError {
    NonAlphabeticCharacters,
    EmptySecretWord,
}

impl Display for SecretWordError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            SecretWordError::NonAlphabeticCharacters => {
                write!(f, "Secret word must contain only alphabetic characters.")
            }
            SecretWordError::EmptySecretWord => write!(f, "Secret word cannot be empty."),
        }
    }
}

impl Error for SecretWordError {}
