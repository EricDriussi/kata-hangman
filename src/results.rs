#[derive(Debug, PartialEq, Eq)]
pub enum GuessResult {
    Correct,
    Incorrect,
    Duplicate,
    Invalid,
}
