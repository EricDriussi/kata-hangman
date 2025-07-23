use crate::errors::InitError;
pub use crate::results::GuessResult;
use crate::results::InitResult;
use crate::state::GameState;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq)]
pub struct Hangman {
    secret_word: Vec<(char, bool)>,
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
            .iter()
            .any(|(secret_char, _)| secret_char.eq(&upper_char))
        {
            self.guessed_chars.insert(upper_char, true);
            for (key, value) in & mut self.secret_word {
                if *key == upper_char {
                    *value = true;
                }
            }
            if self.secret_word
                .iter()
                .all(|(_, guessed)| *guessed) {
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

    pub fn remaining_failures(&self) -> usize {
        self.allowed_failures - self.total_failures
    }

    pub fn already_guessed(&self) -> String {
        let (correct_guesses, incorrect_guesses): (Vec<_>, Vec<_>) = self
            .guessed_chars
            .iter()
            .partition(|&(_, &was_successful)| was_successful);

        format!(
            "\n\tCorrect guesses: {}\n\tIncorrect guesses: {}",
            Self::format_guesses(correct_guesses),
            Self::format_guesses(incorrect_guesses)
        )
    }

    fn format_guesses(chars: Vec<(&char, &bool)>) -> String {
        chars
            .into_iter()
            .map(|(c, _)| c.to_string())
            .collect::<Vec<String>>()
            .join(", ")
    }

    pub fn display_word(&self) -> String {
        self.secret_word
            .iter()
            .map(Self::non_guessed_chars_as_underscore())
            .collect()
    }

    fn non_guessed_chars_as_underscore() -> fn(&(char, bool)) -> char {
        |(char, guessed)| if *guessed { *char } else { '_' }
    }
}
