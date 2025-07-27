use std::fmt;
use crate::errors::CharError;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Char {
    char: char,
}

impl Char {
    pub fn from(c: char) -> Result<Self, CharError> {
        if !c.is_alphabetic() {
            return Err(CharError::NonAlphabeticChar);
        }
        Ok(Char {
            char: c.to_ascii_uppercase(),
        })
    }

    // TODO: impl eq?
    pub fn matches(&self, char: char) -> bool {
        self.char.eq_ignore_ascii_case(&char)
    }
}

impl fmt::Display for Char {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.char)
    }
}
