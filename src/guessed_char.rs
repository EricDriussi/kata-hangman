use crate::alphabetic_char::AlphabeticChar;

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum GuessedChar {
    Correct(AlphabeticChar),
    Incorrect(AlphabeticChar),
}

impl GuessedChar {
    pub fn correct(alphabetic_char: AlphabeticChar) -> Self {
        Self::Correct(alphabetic_char)
    }

    pub fn incorrect(alphabetic_char: AlphabeticChar) -> Self {
        Self::Incorrect(alphabetic_char)
    }

    fn char(&self) -> &AlphabeticChar {
        match self {
            Self::Correct(c) | Self::Incorrect(c) => c,
        }
    }
}

impl PartialEq<AlphabeticChar> for GuessedChar {
    fn eq(&self, other: &AlphabeticChar) -> bool {
        self.char().eq(other)
    }
}
