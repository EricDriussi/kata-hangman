use crate::errors::{AllowedFailuresError, SecretWordError};
use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug, PartialEq, Eq)]
pub enum StartError {
    InvalidAllowedFailures(AllowedFailuresError),
    InvalidSecretWord(SecretWordError),
}

impl Display for StartError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            StartError::InvalidAllowedFailures(e) => write!(f, "{e}"),
            StartError::InvalidSecretWord(e) => write!(f, "{e}"),
        }
    }
}

impl Error for StartError {}

impl From<AllowedFailuresError> for StartError {
    fn from(error: AllowedFailuresError) -> Self {
        StartError::InvalidAllowedFailures(error)
    }
}

impl From<SecretWordError> for StartError {
    fn from(error: SecretWordError) -> Self {
        StartError::InvalidSecretWord(error)
    }
}
