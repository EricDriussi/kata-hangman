use crate::errors::StartError;
use crate::failures::AllowedFailures;
use crate::results::GuessResult;
use crate::secret_word::SecretWord;
use crate::states::GameState;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq)]
pub struct Hangman {
    secret_word: SecretWord,
    failures: AllowedFailures,
    // TODO: Should this be a GuessedChars VO?
    guessed_chars: HashMap<char, bool>,
    state: GameState,
}

impl Hangman {
    pub fn start(word: &str, allowed_failures: isize) -> Result<Hangman, StartError> {
        Ok(Hangman {
            failures: AllowedFailures::from(allowed_failures)?,
            guessed_chars: HashMap::new(),
            secret_word: SecretWord::from(word)?,
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

        if self.secret_word.contains(upper_char) {
            self.guessed_chars.insert(upper_char, true);
            self.secret_word.reveal(upper_char);
            if self.secret_word.is_revealed() {
                self.state = GameState::Won;
            }
            GuessResult::Correct
        } else {
            self.guessed_chars.insert(upper_char, false);
            self.failures.consume();
            if !self.failures.any_left() {
                self.state = GameState::Lost;
            }
            GuessResult::Incorrect
        }
    }

    pub fn remaining_failures(&self) -> isize {
        self.failures.remaining()
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
        self.secret_word.to_string()
    }
}
