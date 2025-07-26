use crate::errors::SecretCharError;

#[derive(Debug, PartialEq, Eq)]
pub struct SecretChar {
    char: char,
    guessed: bool,
}

impl SecretChar {
    pub fn from(c: char) -> Result<Self, SecretCharError> {
        if !c.is_alphabetic() {
            return Err(SecretCharError::NonAlphabeticChar);
        }
        Ok(SecretChar {
            char: c.to_ascii_uppercase(),
            guessed: false,
        })
    }

    pub fn display(&self) -> char {
        if self.guessed {
            self.char
        } else {
            '_'
        }
    }

    pub fn matches(&self, char: char) -> bool {
        self.char.eq_ignore_ascii_case(&char)
    }

    pub fn reveal(&mut self) {
        self.guessed = true;
    }

    pub fn is_guessed(&self) -> bool {
        self.guessed
    }
}
