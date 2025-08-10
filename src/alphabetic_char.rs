use crate::errors::CharError;
use crate::guessed_char::GuessedChar;
use std::fmt;

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct AlphabeticChar {
    char: char,
}

impl AlphabeticChar {
    pub fn from(c: char) -> Result<Self, CharError> {
        if !c.is_alphabetic() {
            return Err(CharError::NonAlphabeticChar);
        }
        Ok(AlphabeticChar {
            char: c.to_ascii_uppercase(),
        })
    }
}

impl fmt::Display for AlphabeticChar {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.char)
    }
}

impl<'a> From<&'a GuessedChar> for &'a AlphabeticChar {
    fn from(guessed: &'a GuessedChar) -> Self {
        match guessed {
            GuessedChar::Correct(c) | GuessedChar::Incorrect(c) => c,
        }
    }
}
