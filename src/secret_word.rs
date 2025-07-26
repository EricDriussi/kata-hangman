use crate::errors::SecretWordError;
use crate::secret_char::SecretChar;

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
                .map(SecretChar::from)
                .collect::<Result<Vec<_>, _>>()?,
        })
    }

    pub fn contains(&self, char: char) -> bool {
        self.word
            .iter()
            .any(|secret_char| secret_char.matches(char))
    }

    pub fn reveal(&mut self, char: char) {
        self.word
            .iter_mut()
            .filter(|secret_char| secret_char.matches(char))
            .for_each(SecretChar::reveal);
    }

    pub fn is_revealed(&self) -> bool {
        self.word.iter().all(SecretChar::is_guessed)
    }

    pub fn display(&self) -> String {
        self.word
            .iter()
            .map(SecretChar::display)
            .collect::<String>()
    }
}
