use crate::alphabetic_char::AlphabeticChar;

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct GuessedChar {
    alphabetic_char: AlphabeticChar,
    guessed_correctly: bool,
}

// TODO: impl From<GuessedChar> for Char {}?
impl GuessedChar {
    pub fn correct(alphabetic_char: AlphabeticChar) -> Self {
        GuessedChar {
            alphabetic_char,
            guessed_correctly: true,
        }
    }

    pub fn incorrect(alphabetic_char: AlphabeticChar) -> Self {
        GuessedChar {
            alphabetic_char,
            guessed_correctly: false,
        }
    }

    pub fn was_correct(&self) -> bool {
        self.guessed_correctly
    }

    pub fn was_incorrect(&self) -> bool {
        !self.guessed_correctly
    }

    pub fn char(&self) -> &AlphabeticChar {
        &self.alphabetic_char
    }
}

impl PartialEq<AlphabeticChar> for GuessedChar {
    fn eq(&self, other: &AlphabeticChar) -> bool {
        self.alphabetic_char.eq(other)
    }
}
