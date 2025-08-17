use crate::failures::AllowedFailures;
use crate::hangman::running_hangman::RunningHangman;
use crate::secret_word::SecretWord;
use std::fmt;

pub struct StoppedHangman {
    secret_word: SecretWord,
    failures: AllowedFailures,
}

impl From<RunningHangman> for StoppedHangman {
    fn from(mut running_game: RunningHangman) -> Self {
        running_game.secret_word.reveal_word();
        StoppedHangman {
            failures: running_game.failures,
            secret_word: running_game.secret_word,
        }
    }
}

impl fmt::Display for StoppedHangman {
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
