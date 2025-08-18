use crate::hangman::running::RunningHangman;
use crate::secret_word::SecretWord;
use std::fmt;

pub struct StoppedHangman {
    secret_word: SecretWord,
    remaining_failures: usize,
}

impl From<RunningHangman> for StoppedHangman {
    fn from(mut running_game: RunningHangman) -> Self {
        running_game.secret_word.reveal_word();
        StoppedHangman {
            remaining_failures: running_game.guessed_chars.remaining(),
            secret_word: running_game.secret_word,
        }
    }
}

impl fmt::Display for StoppedHangman {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let failures_msg = match self.remaining_failures {
            0 => String::new(),
            1 => "You could have failed one more guess\n".to_string(),
            _ => format!(
                "You could have failed {} more guesses\n",
                self.remaining_failures
            ),
        };

        write!(f, "Secret word was: {}\n{}", self.secret_word, failures_msg,)
    }
}
