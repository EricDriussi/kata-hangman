use crate::errors::InitError;
use crate::hangman::Hangman;

#[derive(Debug, PartialEq, Eq)]
pub enum GuessResult {
    Correct,
    Incorrect,
    Duplicate,
    InvalidCharacter,
    GameNotInProgress,
}

pub type InitResult = Result<Hangman, InitError>;
