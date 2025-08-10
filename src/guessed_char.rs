use crate::alphabetic_char::AlphabeticChar;

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct GuessedChar {
    char: AlphabeticChar,
    guessed_correctly: bool,
}

// TODO: impl From<GuessedChar> for Char {}?
impl GuessedChar {
    pub fn correct(char: AlphabeticChar) -> Self {
        GuessedChar {
            char,
            guessed_correctly: true,
        }
    }

    pub fn incorrect(char: AlphabeticChar) -> Self {
        GuessedChar {
            char,
            guessed_correctly: false,
        }
    }

    pub fn matches(&self, char: &AlphabeticChar) -> bool {
        self.char.eq(char)
    }

    pub fn was_correct(&self) -> bool {
        self.guessed_correctly
    }

    pub fn was_incorrect(&self) -> bool {
        !self.guessed_correctly
    }

    pub fn char(&self) -> &AlphabeticChar {
        &self.char
    }
}
