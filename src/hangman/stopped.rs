use crate::chars::alphabetic::AlphabeticChar;
use crate::hangman::running::RunningHangman;
use crate::secret_word::SecretWord;
use std::fmt;

pub struct StoppedHangman {
    secret_word: SecretWord,
    remaining_failures: usize,
}

impl StoppedHangman {
    fn new(secret_word: SecretWord, remaining_failures: usize) -> Self {
        let mut stopped = Self {
            secret_word,
            remaining_failures,
        };
        stopped.brute_force_word();
        stopped
    }

    fn brute_force_word(&mut self) {
        let char_min = '\u{0000}';
        let char_max = char::MAX;
        (char_min..=char_max)
            .filter_map(|c| AlphabeticChar::from(c).ok())
            .filter(|c| self.secret_word.contains(c))
            .collect::<Vec<_>>() // Filters are lazy, need to collect to close its borrow, seems wasteful
            .into_iter()
            .for_each(|c| self.secret_word.reveal_char(&c));
    }
}

impl From<RunningHangman> for StoppedHangman {
    fn from(running_game: RunningHangman) -> Self {
        StoppedHangman::new(
            running_game.secret_word,
            running_game.guessed_chars.remaining(),
        )
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
