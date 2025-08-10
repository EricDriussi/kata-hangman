use crate::alphabetic_char::AlphabeticChar;
use crate::guessed_char::GuessedChar;
use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq)]
pub struct GuessedChars {
    chars: HashSet<GuessedChar>,
}

impl GuessedChars {
    pub fn none() -> Self {
        GuessedChars {
            chars: HashSet::new(),
        }
    }

    pub fn add_correct(&mut self, char: AlphabeticChar) {
        self.chars.insert(GuessedChar::correct(char));
    }

    pub fn add_incorrect(&mut self, char: AlphabeticChar) {
        self.chars.insert(GuessedChar::incorrect(char));
    }

    pub fn already_guessed(&self, char: &AlphabeticChar) -> bool {
        self.chars
            .iter()
            .any(|guessed_char| guessed_char.eq(char))
    }

    pub fn correct_guesses(&self) -> HashSet<&AlphabeticChar> {
        self.chars
            .iter()
            .filter(|guessed_char| guessed_char.was_correct())
            .map(GuessedChar::char)
            .collect()
    }

    pub fn incorrect_guesses(&self) -> HashSet<&AlphabeticChar> {
        self.chars
            .iter()
            .filter(|guessed_char| guessed_char.was_incorrect())
            .map(GuessedChar::char)
            .collect()
    }
}
