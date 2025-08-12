use crate::hangman::generic_hangman::{Hangman, Running, Stopped};

pub enum GameState {
    InProgress(Hangman<Running>),
    Won(Hangman<Stopped>),
    Lost(Hangman<Stopped>),
}
