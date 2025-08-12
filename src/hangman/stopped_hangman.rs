use crate::hangman::generic_hangman::{Hangman, Running, Stopped};
use std::fmt;
use std::marker::PhantomData;

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
