use crate::alphabetic_char::AlphabeticChar;
use crate::errors::StartError;
use crate::failures::AllowedFailures;
use crate::guessed_chars::GuessedChars;
use crate::results::GuessResult;
use crate::secret_word::SecretWord;
use std::collections::HashSet;
use std::fmt;
use std::marker::PhantomData;

pub trait HangmanType {}

pub struct Running;
impl HangmanType for Running {}

pub struct Stopped;
impl HangmanType for Stopped {}

pub struct Hangman<S: HangmanType> {
    _marker: PhantomData<S>,
    secret_word: SecretWord,
    failures: AllowedFailures,
    guessed_chars: GuessedChars,
}

impl<S: HangmanType> Hangman<S> {
    pub fn start(word: &str, allowed_failures: isize) -> Result<Hangman<Running>, StartError> {
        Ok(Hangman {
            _marker: PhantomData,
            failures: AllowedFailures::from(allowed_failures)?,
            guessed_chars: GuessedChars::none(),
            secret_word: SecretWord::from(word)?,
        })
    }
}

pub enum GameState {
    InProgress(Hangman<Running>),
    Won(Hangman<Stopped>),
    Lost(Hangman<Stopped>),
}

impl Hangman<Running> {
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
                return (GuessResult::Correct, GameState::Won(Hangman::from(self)));
            }
            (GuessResult::Correct, GameState::InProgress(self))
        } else {
            self.guessed_chars.add_incorrect(char);
            self.failures.consume();
            if !self.failures.any_left() {
                return (GuessResult::Incorrect, GameState::Lost(Hangman::from(self)));
            }
            (GuessResult::Incorrect, GameState::InProgress(self))
        }
    }

    fn already_guessed(&self) -> String {
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
}

impl fmt::Display for Hangman<Running> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Secret word: {}\nRemaining failures: {}\nAlready guessed characters: {}",
            self.secret_word,
            self.failures.remaining(),
            self.already_guessed()
        )
    }
}

impl fmt::Display for Hangman<Stopped> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let remaining_failures = match self.failures.remaining() {
            0 => String::new(),
            1 => "You could have failed one more guess\n".to_string(),
            _ => format!(
                "You could have failed {} more guesses\n",
                self.failures.remaining()
            ),
        };

        write!(
            f,
            "Secret word was: {}\n{}",
            self.secret_word, remaining_failures,
        )
    }
}

impl From<Hangman<Running>> for Hangman<Stopped> {
    fn from(mut running_game: Hangman<Running>) -> Self {
        running_game.secret_word.reveal_word();
        Hangman {
            _marker: PhantomData,
            failures: running_game.failures,
            guessed_chars: running_game.guessed_chars,
            secret_word: running_game.secret_word,
        }
    }
}
