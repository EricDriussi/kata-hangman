use crate::alphabetic_char::AlphabeticChar;
use crate::errors::{GuessError, StartError};
use crate::failures::AllowedFailures;
use crate::guessed_chars::GuessedChars;
use crate::results::GuessResult;
use crate::secret_word::SecretWord;
use crate::states::GameState;
use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq)]
pub struct Hangman {
    secret_word: SecretWord,
    failures: AllowedFailures,
    guessed_chars: GuessedChars,
    state: GameState,
}

impl Hangman {
    pub fn start(word: &str, allowed_failures: isize) -> Result<Hangman, StartError> {
        Ok(Hangman {
            failures: AllowedFailures::from(allowed_failures)?,
            guessed_chars: GuessedChars::none(),
            secret_word: SecretWord::from(word)?,
            state: GameState::InProgress,
        })
    }

    pub fn state(&self) -> GameState {
        self.state
    }

    pub fn guess(&mut self, character: char) -> Result<GuessResult, GuessError> {
        if self.state != GameState::InProgress {
            return Err(GuessError::GameNotInProgress);
        }

        let char = AlphabeticChar::from(character)?;

        if self.guessed_chars.already_guessed(&char) {
            return Ok(GuessResult::Duplicate);
        }

        if self.secret_word.contains(&char) {
            self.secret_word.reveal(&char);
            self.guessed_chars.add_correct(char);
            if self.secret_word.is_revealed() {
                self.state = GameState::Won;
            }
            Ok(GuessResult::Correct)
        } else {
            self.guessed_chars.add_incorrect(char);
            self.failures.consume();
            if !self.failures.any_left() {
                self.state = GameState::Lost;
            }
            Ok(GuessResult::Incorrect)
        }
    }

    pub fn remaining_failures(&self) -> isize {
        self.failures.remaining()
    }

    pub fn already_guessed(&self) -> String {
        format!(
            "\n\tCorrect guesses: {}\n\tIncorrect guesses: {}",
            Self::format_guesses(self.guessed_chars.correct_guesses()),
            Self::format_guesses(self.guessed_chars.incorrect_guesses())
        )
    }

    fn format_guesses(chars: HashSet<&AlphabeticChar>) -> String {
        chars
            .into_iter()
            .map(ToString::to_string)
            .collect::<Vec<String>>()
            .join(", ")
    }

    pub fn display_word(&self) -> String {
        self.secret_word.to_string()
    }
}
