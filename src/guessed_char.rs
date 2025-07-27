use crate::char::Char;

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct GuessedChar {
    char: Char,
    guessed_correctly: bool,
}

// TODO: impl From<GuessedChar> for Char {}?
impl GuessedChar {
    pub fn correct(char: Char) -> Self {
        GuessedChar {
            char,
            guessed_correctly: true,
        }
    }

    pub fn incorrect(char: Char) -> Self {
        GuessedChar {
            char,
            guessed_correctly: false,
        }
    }

    pub fn matches(&self, char: &Char) -> bool {
        self.char.eq(char)
    }

    pub fn was_correct(&self) -> bool {
        self.guessed_correctly
    }

    pub fn was_incorrect(&self) -> bool {
        !self.guessed_correctly
    }

    pub fn char(&self) -> &Char {
        &self.char
    }
}
