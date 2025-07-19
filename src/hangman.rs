use crate::errors::InitError;
pub use crate::results::GuessResult;
use crate::results::InitResult;
use crate::state::GameState;
use indexmap::IndexMap;
use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq)]
pub struct Hangman {
    word: String,
    max_failed_guesses: usize,
    failed_guesses: usize,
    guessed_letters: HashSet<char>,
    word_progress: IndexMap<char, bool>,
    state: GameState,
}

impl Hangman {
    pub fn init(word: &str, max_failed_guesses: usize) -> InitResult {
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
        if self.state != GameState::InProgress {
            return GuessResult::GameNotInProgress;
        }

        if !letter.is_alphabetic() {
            return GuessResult::InvalidCharacter;
        }

        let upper_letter = letter.to_uppercase().next().unwrap_or(letter);

        if self.guessed_letters.contains(&upper_letter) {
            return GuessResult::Duplicate;
        }

        if self.word.contains(upper_letter) {
            self.guessed_letters.insert(upper_letter);
            self.word_progress.insert(upper_letter, true);
            if self.word_progress.values().all(|&guessed| guessed) {
                self.state = GameState::Won;
            }
            GuessResult::Correct
        } else {
            self.failed_guesses += 1;
            if self.failed_guesses >= self.max_failed_guesses {
                self.state = GameState::Lost;
            }
            GuessResult::Incorrect
        }
    }

    pub fn display_word(&self) -> String {
        self.word_progress
            .iter()
            .map(Self::non_guessed_chars_as_underscore())
            .collect()
    }

    fn non_guessed_chars_as_underscore() -> fn((&char, &bool)) -> char {
        |(&char, &guessed)| if guessed { char } else { '_' }
    }
}
