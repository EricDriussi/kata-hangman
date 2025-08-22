use crate::chars::error::CharError;
use std::error::Error;
use std::fmt::{Display, Formatter, Result};

#[derive(Debug, PartialEq, Eq)]
pub enum AllowedFailuresError {
    NotEnough,
}

impl Display for AllowedFailuresError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            AllowedFailuresError::NotEnough => {
                write!(f, "Must allow at least one failure.")
            }
        }
    }
}

impl Error for AllowedFailuresError {}

#[derive(Debug, PartialEq, Eq)]
pub enum SecretWordError {
    NonAlphabeticChars,
    Empty,
}

impl Display for SecretWordError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            SecretWordError::NonAlphabeticChars => {
                write!(f, "Secret word must contain only alphabetic characters.")
            }
            SecretWordError::Empty => write!(f, "Secret word cannot be empty."),
        }
    }
}

impl From<CharError> for SecretWordError {
    fn from(_error: CharError) -> Self {
        SecretWordError::NonAlphabeticChars
    }
}

impl Error for SecretWordError {}
