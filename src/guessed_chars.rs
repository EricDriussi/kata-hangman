use crate::chars::alphabetic::AlphabeticChar;
use crate::chars::guessed::GuessedChar;
use crate::errors::AllowedFailuresError;
use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq)]
pub struct GuessedChars {
    guessed_chars: HashSet<GuessedChar>,
    allowed_failures: usize,
}

impl GuessedChars {
    pub fn allowed_failures(limit: usize) -> Result<Self, AllowedFailuresError> {
        if limit < 1 {
            return Err(AllowedFailuresError::NotEnough);
        }

        Ok(GuessedChars {
            guessed_chars: HashSet::new(),
            allowed_failures: limit,
        })
    }

    pub fn add_correct(&mut self, alphabetic_char: AlphabeticChar) {
        self.guessed_chars
            .insert(GuessedChar::Correct(alphabetic_char));
    }

    pub fn add_failed(&mut self, alphabetic_char: AlphabeticChar) {
        self.guessed_chars
            .insert(GuessedChar::Incorrect(alphabetic_char));
    }

    pub fn remaining(&self) -> usize {
        self.allowed_failures
            - self
                .guessed_chars
                .iter()
                .filter(|g| matches!(g, GuessedChar::Incorrect(_)))
                .count()
    }

    pub fn no_failures_available(&self) -> bool {
        self.remaining() == 0
    }

    pub fn already_guessed(&self, alphabetic_char: &AlphabeticChar) -> bool {
        self.guessed_chars
            .iter()
            .any(|guessed_char| guessed_char.eq(alphabetic_char))
    }

    pub fn correct_guesses(&self) -> HashSet<&AlphabeticChar> {
        self.guessed_chars
            .iter()
            .filter(|g| matches!(g, GuessedChar::Correct(_)))
            .map(Into::into)
            .collect()
    }

    pub fn failed_guesses(&self) -> HashSet<&AlphabeticChar> {
        self.guessed_chars
            .iter()
            .filter(|g| matches!(g, GuessedChar::Incorrect(_)))
            .map(Into::into)
            .collect()
    }
}
