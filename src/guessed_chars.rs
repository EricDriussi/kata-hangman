use crate::alphabetic_char::AlphabeticChar;
use crate::guessed_char::GuessedChar;
use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq)]
pub struct GuessedChars {
    guessed_chars: HashSet<GuessedChar>,
}

impl GuessedChars {
    pub fn none() -> Self {
        GuessedChars {
            guessed_chars: HashSet::new(),
        }
    }

    pub fn add_correct(&mut self, alphabetic_char: AlphabeticChar) {
        self.guessed_chars
            .insert(GuessedChar::Correct(alphabetic_char));
    }

    pub fn add_incorrect(&mut self, alphabetic_char: AlphabeticChar) {
        self.guessed_chars
            .insert(GuessedChar::Incorrect(alphabetic_char));
    }

    pub fn already_guessed(&self, alphabetic_char: &AlphabeticChar) -> bool {
        self.guessed_chars
            .iter()
            .any(|guessed_char| guessed_char.eq(alphabetic_char))
    }

    // TODO: should return HashSet<GuessedChar::Correct>?
    pub fn correct_guesses(&self) -> HashSet<&AlphabeticChar> {
        self.guessed_chars
            .iter()
            .filter(|g| matches!(g, GuessedChar::Correct(_)))
            .map(Into::into)
            .collect()
    }

    // TODO: should return HashSet<GuessedChar::Incorrect>?
    pub fn incorrect_guesses(&self) -> HashSet<&AlphabeticChar> {
        self.guessed_chars
            .iter()
            .filter(|g| matches!(g, GuessedChar::Incorrect(_)))
            .map(Into::into)
            .collect()
    }

    // TODO: maybe a guesses(&self) -> (HashSet<GuessedChar::Correct>, HashSet<GuessedChar::Incorrect>) function instead of these two??
}
