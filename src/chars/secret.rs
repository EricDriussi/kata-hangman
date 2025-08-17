use crate::chars::alphabetic::AlphabeticChar;
use std::fmt;

#[derive(Debug, PartialEq, Eq)]
pub struct SecretChar {
    alphabetic_char: AlphabeticChar,
    hidden: bool,
}

impl SecretChar {
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
            write!(f, "{}", self.alphabetic_char)
        }
    }
}

impl From<AlphabeticChar> for SecretChar {
    fn from(char: AlphabeticChar) -> Self {
        SecretChar { alphabetic_char: char, hidden: true }
    }
}

impl PartialEq<AlphabeticChar> for SecretChar {
    fn eq(&self, other: &AlphabeticChar) -> bool {
        self.alphabetic_char.eq(other)
    }
}

impl PartialEq<AlphabeticChar> for &mut SecretChar {
    fn eq(&self, other: &AlphabeticChar) -> bool {
        self.alphabetic_char.eq(other)
    }
}
