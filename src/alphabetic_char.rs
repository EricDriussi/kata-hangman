use std::fmt;
use crate::errors::CharError;

#[derive(Debug, Eq, Hash, Clone)]
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

impl PartialEq for AlphabeticChar {
    fn eq(&self, other: &Self) -> bool {
        self.char.eq_ignore_ascii_case(&other.char)
    }
}

impl fmt::Display for AlphabeticChar {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.char)
    }
}
