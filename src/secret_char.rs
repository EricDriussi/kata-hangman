use crate::char::Char;
use crate::errors::CharError;
use std::fmt;

#[derive(Debug, PartialEq, Eq)]
pub struct SecretChar {
    char: Char,
    hidden: bool,
}

impl SecretChar {
    pub fn from(char: Char) -> Result<Self, CharError> {
        Ok(SecretChar {
            char,
            hidden: true,
        })
    }

    pub fn matches(&self, char: char) -> bool {
        self.char.matches(char)
    }

    pub fn reveal(&mut self) {
        self.hidden = false;
    }

    pub fn is_guessed(&self) -> bool {
        !self.hidden
    }
}

impl fmt::Display for SecretChar {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.hidden {
            write!(f, "_")
        } else {
            write!(f, "{}", self.char)
        }
    }
}
