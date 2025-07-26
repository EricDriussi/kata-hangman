use crate::char::Char;
use crate::errors::SecretWordError;
use crate::secret_char::SecretChar;
use std::fmt;

#[derive(Debug, PartialEq, Eq)]
pub struct SecretWord {
    word: Vec<SecretChar>,
}

impl SecretWord {
    pub fn from(word: &str) -> Result<Self, SecretWordError> {
        if word.is_empty() {
            return Err(SecretWordError::EmptySecretWord);
        }

        Ok(SecretWord {
            word: word
                .to_uppercase()
                .chars()
                .map(Char::from)
                .map(|char| char.and_then(SecretChar::from))
                .collect::<Result<Vec<_>, _>>()?,
        })
    }

    pub fn contains(&self, char: &Char) -> bool {
        self.word
            .iter()
            .any(|secret_char| secret_char.matches(char))
    }

    pub fn reveal(&mut self, char: &Char) {
        self.word
            .iter_mut()
            .filter(|secret_char| secret_char.matches(char))
            .for_each(SecretChar::reveal);
    }

    pub fn is_revealed(&self) -> bool {
        self.word.iter().all(SecretChar::is_guessed)
    }
}

impl fmt::Display for SecretWord {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let word = self
            .word
            .iter()
            .map(SecretChar::to_string)
            .collect::<String>();
        write!(f, "{word}")
    }
}
