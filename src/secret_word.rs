use crate::alphabetic_char::AlphabeticChar;
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
                .map(AlphabeticChar::from)
                .map(|char| char.map(SecretChar::from))
                .collect::<Result<Vec<SecretChar>, _>>()?,
        })
    }

    pub fn contains(&self, alphabetic_char: &AlphabeticChar) -> bool {
        self.word
            .iter()
            .any(|secret_char| secret_char.eq(alphabetic_char))
    }

    pub fn reveal(&mut self, alphabetic_char: &AlphabeticChar) {
        self.word
            .iter_mut()
            .filter(|secret_char| secret_char.eq(alphabetic_char))
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
