use crate::chars::alphabetic::AlphabeticChar;
use crate::game_state::GameState;
use crate::guessed_chars::GuessedChars;
use crate::hangman::stopped::StoppedHangman;
use crate::results::GuessResult;
use crate::secret_word::SecretWord;
use std::collections::HashSet;
use std::fmt;

pub struct RunningHangman {
    pub(crate) secret_word: SecretWord,
    pub(crate) guessed_chars: GuessedChars,
}

impl RunningHangman {
    pub fn guess(mut self, character: char) -> (GuessResult, GameState) {
        let Ok(char) = AlphabeticChar::from(character) else {
            return (GuessResult::Invalid, GameState::InProgress(self));
        };

        if self.guessed_chars.already_guessed(&char) {
            return (GuessResult::Duplicate, GameState::InProgress(self));
        }

        if self.secret_word.contains(&char) {
            self.secret_word.reveal_char(&char);
            self.guessed_chars.add_correct(char);
            if self.secret_word.is_revealed() {
                return (
                    GuessResult::Correct,
                    GameState::Won(StoppedHangman::from(self)),
                );
            }
            (GuessResult::Correct, GameState::InProgress(self))
        } else {
            self.guessed_chars.add_failed(char);
            if self.guessed_chars.no_failures_available() {
                return (
                    GuessResult::Incorrect,
                    GameState::Lost(StoppedHangman::from(self)),
                );
            }
            (GuessResult::Incorrect, GameState::InProgress(self))
        }
    }

    fn already_guessed(&self) -> String {
        format!(
            "\n\tCorrect guesses: {}\n\tFailed guesses: {}",
            Self::format_guesses(self.guessed_chars.correct_guesses()),
            Self::format_guesses(self.guessed_chars.failed_guesses())
        )
    }

    fn format_guesses(chars: HashSet<&AlphabeticChar>) -> String {
        chars
            .into_iter()
            .map(ToString::to_string)
            .collect::<Vec<String>>()
            .join(", ")
    }
}

impl fmt::Display for RunningHangman {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Secret word: {}\nRemaining failures: {}\nAlready guessed characters: {}",
            self.secret_word,
            self.guessed_chars.remaining(),
            self.already_guessed()
        )
    }
}
