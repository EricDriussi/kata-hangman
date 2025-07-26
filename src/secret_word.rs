use crate::errors::SecretWordError;

#[derive(Debug, PartialEq, Eq)]
pub struct SecretWord {
    // TODO: Should this be a Vec<SecretChar>?
    secret_chars: Vec<(char, bool)>,
}

impl SecretWord {
    pub fn from(word: &str) -> Result<Self, SecretWordError> {
        if word.is_empty() {
            return Err(SecretWordError::EmptySecretWord);
        }

        if word.chars().any(|ch| !ch.is_alphabetic()) {
            return Err(SecretWordError::NonAlphabeticCharacters);
        }
        Ok(SecretWord {
            secret_chars: word.to_uppercase().chars().map(|ch| (ch, false)).collect(),
        })
    }

    pub fn contains(&self, character: char) -> bool {
        self.secret_chars
            .iter()
            .any(|(secret_char, _)| secret_char.eq_ignore_ascii_case(&character))
    }

    pub fn reveal(&mut self, character: char) {
        for (key, value) in &mut self.secret_chars {
            if key.eq_ignore_ascii_case(&character) {
                *value = true;
            }
        }
    }

    pub fn is_revealed(&self) -> bool {
        self.secret_chars.iter().all(|(_, guessed)| *guessed)
    }

    pub fn display(&self) -> String {
        self.secret_chars
            .iter()
            .map(Self::non_guessed_chars_as_underscore())
            .collect()
    }

    fn non_guessed_chars_as_underscore() -> fn(&(char, bool)) -> char {
        |(char, guessed)| if *guessed { *char } else { '_' }
    }
}
