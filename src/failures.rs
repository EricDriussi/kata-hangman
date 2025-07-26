use crate::errors::AllowedFailuresError;

#[derive(Debug, PartialEq, Eq)]
pub struct AllowedFailures {
    remaining: isize,
}

impl AllowedFailures {
    pub fn from(limit: isize) -> Result<Self, AllowedFailuresError> {
        if limit < 1 {
            return Err(AllowedFailuresError::NotEnoughGuesses);
        }

        Ok(AllowedFailures { remaining: limit })
    }

    pub fn remaining(&self) -> isize {
        self.remaining
    }

    pub fn consume(&mut self) {
        self.remaining -= 1;
    }

    pub fn any_left(&self) -> bool {
        self.remaining > 0
    }
}
