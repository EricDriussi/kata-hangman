use crate::helpers::invalid_chars;
use hangman::hangman::factory::Hangman;
use hangman::results::GuessResult;
use rstest::rstest;
use rstest_reuse::apply;

const VALID_ALLOWED_FAILURES: usize = 1;
const VALID_WORD: &str = "aWordñ";

#[test]
fn starts_with_valid_word_and_limit() {
    let game = Hangman::start(VALID_WORD, VALID_ALLOWED_FAILURES);

    assert!(game.is_ok());
}

#[apply(invalid_chars)]
fn does_not_accept_invalid_guesses(#[case] invalid_char: char) {
    let game = Hangman::start(VALID_WORD, VALID_ALLOWED_FAILURES).unwrap();

    let (guess_result, _) = game.guess(invalid_char);

    assert!(matches!(guess_result, GuessResult::Invalid));
}
