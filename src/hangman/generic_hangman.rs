use crate::failures::AllowedFailures;
use crate::guessed_chars::GuessedChars;
use crate::secret_word::SecretWord;
use std::marker::PhantomData;

pub trait HangmanType {}

pub struct Running;
impl HangmanType for Running {}

pub struct Stopped;
impl HangmanType for Stopped {}

pub struct Hangman<S: HangmanType> {
    pub(crate) _marker: PhantomData<S>,
    pub(crate) secret_word: SecretWord,
    pub(crate) failures: AllowedFailures,
    pub(crate) guessed_chars: GuessedChars,
}
