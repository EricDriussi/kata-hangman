use crate::alphabetic_char::AlphabeticChar;
use crate::errors::{GuessError, StartError};
use crate::failures::AllowedFailures;
use crate::guessed_chars::GuessedChars;
use crate::results::GuessResult;
use crate::secret_word::SecretWord;
use crate::states::GameState;
use std::collections::HashSet;
use std::marker::PhantomData;

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

        let Ok(char) = AlphabeticChar::from(character) else {
            return Ok(GuessResult::Invalid);
        };

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

pub trait GameState2 {}

pub struct RunningGame;
impl GameState2 for RunningGame {}

pub struct StoppedGame;
impl GameState2 for StoppedGame {}

pub struct Hangman2<S: GameState2> {
    _marker: PhantomData<S>,
    secret_word: SecretWord,
    failures: AllowedFailures,
    guessed_chars: GuessedChars,
}

impl<S: GameState2> Hangman2<S> {
    pub fn start(word: &str, allowed_failures: isize) -> Result<Hangman2<RunningGame>, StartError> {
        Ok(Hangman2 {
            _marker: PhantomData,
            failures: AllowedFailures::from(allowed_failures)?,
            guessed_chars: GuessedChars::none(),
            secret_word: SecretWord::from(word)?,
        })
    }
}

pub enum HangmanState {
    Running(Hangman2<RunningGame>),
    Stopped(Hangman2<StoppedGame>),
}

impl Hangman2<RunningGame> {
    pub fn guess(mut self, character: char) -> (GuessResult, HangmanState) {
        let Ok(char) = AlphabeticChar::from(character) else {
            return (GuessResult::Invalid, HangmanState::Running(self));
        };

        if self.guessed_chars.already_guessed(&char) {
            return (GuessResult::Duplicate, HangmanState::Running(self));
        }

        if self.secret_word.contains(&char) {
            self.secret_word.reveal(&char);
            self.guessed_chars.add_correct(char);
            if self.secret_word.is_revealed() {
                return (
                    GuessResult::Correct,
                    HangmanState::Stopped(Hangman2 {
                        _marker: PhantomData,
                        failures: self.failures,
                        guessed_chars: self.guessed_chars,
                        secret_word: self.secret_word,
                    }),
                );
            }
            (GuessResult::Correct, HangmanState::Running(self))
        } else {
            self.guessed_chars.add_incorrect(char);
            self.failures.consume();
            if !self.failures.any_left() {
                return (
                    GuessResult::Incorrect,
                    HangmanState::Stopped(Hangman2 {
                        _marker: PhantomData,
                        failures: self.failures,
                        guessed_chars: self.guessed_chars,
                        secret_word: self.secret_word,
                    }),
                );
            }
            (GuessResult::Incorrect, HangmanState::Running(self))
        }
    }
}

impl Hangman2<StoppedGame> {
    // TODO: Actually report the result (win or lost? reveal secret word?)
    pub fn result(&self) {
        if self.secret_word.is_revealed() {
            println!(
                "Congratulations! You guessed the word: {}",
                self.secret_word
            );
        } else {
            println!("Game over! The word was: {}", self.secret_word);
        }
        println!("Remaining failures: {}", self.failures.remaining());
    }
}
