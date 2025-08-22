use crate::hangman::running::RunningHangman;
use crate::hangman::stopped::StoppedHangman;

mod error;
pub mod factory;
pub mod running;
pub mod stopped;

pub enum GameState {
    InProgress(RunningHangman),
    Lost(StoppedHangman),
    Won(StoppedHangman),
}

#[derive(Debug, PartialEq, Eq)]
pub enum GuessResult {
    Correct,
    Incorrect,
    Duplicate,
    Invalid,
}
