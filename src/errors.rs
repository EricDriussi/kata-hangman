use std::error::Error;
use std::fmt::{Display, Formatter, Result};

// TODO: Too much crap in this file, should be split up
// Maybe create modules?

#[derive(Debug, PartialEq, Eq)]
// TODO: Does this StartError make sense? Should it be simpler or just use error::Error?
pub enum StartError {
    EmptySecretWord,
    NonAlphabeticCharacters,
    NotEnoughGuesses,
}

impl Display for StartError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            StartError::EmptySecretWord => write!(f, "Secret word cannot be empty."),
            StartError::NonAlphabeticCharacters => {
                write!(f, "Secret word must contain only alphabetic characters.")
            }
            StartError::NotEnoughGuesses => write!(f, "Incorrect guess limit must be at least 1."),
        }
    }
}

impl Error for StartError {}

impl From<AllowedFailuresError> for StartError {
    fn from(error: AllowedFailuresError) -> Self {
        match error {
            AllowedFailuresError::NotEnoughGuesses => StartError::NotEnoughGuesses,
        }
    }
}

impl From<SecretWordError> for StartError {
    fn from(error: SecretWordError) -> Self {
        match error {
            SecretWordError::EmptySecretWord => StartError::EmptySecretWord,
            SecretWordError::NonAlphabeticCharacters => StartError::NonAlphabeticCharacters,
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

impl From<SecretCharError> for SecretWordError {
    fn from(error: SecretCharError) -> Self {
        match error {
            SecretCharError::NonAlphabeticChar => SecretWordError::NonAlphabeticCharacters,
        }
    }
}

impl Error for SecretWordError {}

#[derive(Debug, PartialEq, Eq)]
pub enum SecretCharError {
    NonAlphabeticChar,
}

impl Display for SecretCharError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            SecretCharError::NonAlphabeticChar => {
                write!(f, "Secret char can only by alphabetic.")
            }
        }
    }
}

impl Error for SecretCharError {}
