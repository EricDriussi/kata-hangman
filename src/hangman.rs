use crate::errors::InitError;
pub use crate::results::GuessResult;
use crate::results::InitResult;
use crate::state::GameState;
use indexmap::IndexMap;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq)]
pub struct Hangman {
    secret_word: IndexMap<char, bool>,
    allowed_failures: usize,
    total_failures: usize,
    guessed_chars: HashMap<char, bool>,
    state: GameState,
}

impl Hangman {
    pub fn init(word: &str, allowed_failures: usize) -> InitResult {
        if allowed_failures < 1 {
            return Err(InitError::NotEnoughGuesses);
        }

        if word.is_empty() {
            return Err(InitError::EmptySecretWord);
        }

        if word.chars().any(|ch| !ch.is_alphabetic()) {
            return Err(InitError::NonAlphabeticCharacters);
        }

        Ok(Hangman {
            allowed_failures,
            total_failures: 0,
            guessed_chars: HashMap::new(),
            secret_word: word.to_uppercase().chars().map(|ch| (ch, false)).collect(),
            state: GameState::InProgress,
        })
    }

    pub fn state(&self) -> GameState {
        self.state
    }

    pub fn guess(&mut self, character: char) -> GuessResult {
        if self.state != GameState::InProgress {
            return GuessResult::GameNotInProgress;
        }

        if !character.is_alphabetic() {
            return GuessResult::InvalidCharacter;
        }

        let upper_char = character.to_uppercase().next().unwrap_or(character);

        if self
            .guessed_chars
            .keys()
            .any(|&secret_char| secret_char.eq(&upper_char))
        {
            return GuessResult::Duplicate;
        }

        if self
            .secret_word
            .keys()
            .any(|&secret_char| secret_char.eq(&upper_char))
        {
            self.guessed_chars.insert(upper_char, true);
            self.secret_word.insert(upper_char, true);
            if self.secret_word.values().all(|&guessed| guessed) {
                self.state = GameState::Won;
            }
            GuessResult::Correct
        } else {
            self.guessed_chars.insert(upper_char, false);
            self.total_failures += 1;
            if self.total_failures >= self.allowed_failures {
                self.state = GameState::Lost;
            }
            GuessResult::Incorrect
        }
    }

    pub fn display_word(&self) -> String {
        self.secret_word
            .iter()
            .map(Self::non_guessed_chars_as_underscore())
            .collect()
    }

    fn non_guessed_chars_as_underscore() -> fn((&char, &bool)) -> char {
        |(&char, &guessed)| if guessed { char } else { '_' }
    }
}
