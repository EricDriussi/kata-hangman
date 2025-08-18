use crate::errors::StartError;
use crate::guessed_chars::GuessedChars;
use crate::hangman::running::RunningHangman;
use crate::secret_word::SecretWord;

pub struct Hangman {}

impl Hangman {
    pub fn start(word: &str, allowed_failures: usize) -> Result<RunningHangman, StartError> {
        Ok(RunningHangman {
            guessed_chars: GuessedChars::allowed_failures(allowed_failures)?,
            secret_word: SecretWord::from(word)?,
        })
    }
}
