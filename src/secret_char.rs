use crate::char::Char;
use crate::errors::CharError;
use std::fmt;

#[derive(Debug, PartialEq, Eq)]
pub struct SecretChar {
    char: Char,
    guessed: bool,
}

impl SecretChar {
    pub fn from(c: char) -> Result<Self, CharError> {
        Ok(SecretChar {
            char: Char::from(c)?,
            guessed: false,
        })
    }

    pub fn matches(&self, char: char) -> bool {
        self.char.matches(char)
    }

    pub fn reveal(&mut self) {
        self.guessed = true;
    }

    pub fn is_guessed(&self) -> bool {
        self.guessed
    }
}

impl fmt::Display for SecretChar {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.guessed {
            write!(f, "{}", self.char)
        } else {
            write!(f, "_")
        }
    }
}
