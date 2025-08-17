use crate::errors::StartError;
use crate::failures::AllowedFailures;
use crate::guessed_chars::GuessedChars;
use crate::hangman::running::RunningHangman;
use crate::secret_word::SecretWord;

pub struct Hangman {}

impl Hangman {
    pub fn start(word: &str, allowed_failures: isize) -> Result<RunningHangman, StartError> {
        Ok(RunningHangman {
            failures: AllowedFailures::from(allowed_failures)?,
            guessed_chars: GuessedChars::none(),
            secret_word: SecretWord::from(word)?,
        })
    }
}
