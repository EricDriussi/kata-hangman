use crate::errors::InitError;
pub use crate::results::GuessResult;
use crate::state::GameState;
use std::collections::{HashMap, HashSet};

#[derive(Debug, PartialEq, Eq)]
pub struct Hangman {
    word: String,
    max_failed_guesses: isize,
    failed_guesses: usize,
    guessed_letters: HashSet<char>,
    word_progress: HashMap<char, bool>,
    state: GameState,
}

impl Hangman {
    pub fn init(word: &str, max_failed_guesses: isize) -> Result<Self, InitError> {
        if max_failed_guesses < 1 {
            return Err(InitError::NotEnoughGuesses);
        }

        if word.is_empty() {
            return Err(InitError::EmptySecretWord);
        }

        if word.chars().any(|ch| !ch.is_alphabetic()) {
            return Err(InitError::NonAlphabeticCharacters);
        }

        Ok(Hangman {
            word: word.to_uppercase().to_string(),
            max_failed_guesses,
            failed_guesses: 0,
            guessed_letters: HashSet::new(),
            word_progress: word.to_uppercase().chars().map(|ch| (ch, false)).collect(),
            state: GameState::InProgress,
        })
    }

    pub fn state(&self) -> GameState {
        self.state
    }

    pub fn guess(&mut self, letter: char) -> GuessResult {
        // TODO: what if the game is not in progress?
        if !letter.is_alphabetic() {
            return GuessResult::InvalidCharacter;
        }

        let upper_letter = letter.to_uppercase().next().unwrap_or(letter);

        if self.guessed_letters.contains(&upper_letter) {
            return GuessResult::Duplicate;
        }

        // TODO: should also update game state
        if self.word.contains(upper_letter) {
            self.guessed_letters.insert(upper_letter);
            GuessResult::Correct
        } else {
            self.failed_guesses += 1;
            GuessResult::Incorrect
        }
    }

    // pub fn display_word(&self) -> String {
    //     self.secret_word
    //         .chars()
    //         .map(|c| {
    //             if *self.revealed_word_state.get(&c).unwrap_or(&false) {
    //                 c
    //             } else {
    //                 '_'
    //             }
    //         })
    //         .collect()
    // }
}
