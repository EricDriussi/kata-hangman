use crate::alphabetic_char::AlphabeticChar;
use std::fmt;

#[derive(Debug, PartialEq, Eq)]
pub struct SecretChar {
    char: AlphabeticChar,
    hidden: bool,
}

impl SecretChar {
    pub fn from(char: AlphabeticChar) -> Self {
        SecretChar {
            char,
            hidden: true,
        }
    }

    // TODO: Implement PartialEq<Char>?
    pub fn matches(&self, char: &AlphabeticChar) -> bool {
        self.char.eq(char)
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
