use crate::helpers::invalid_chars;
use hangman::hangman::factory::Hangman;
use rstest::rstest;
use rstest_reuse::apply;

#[apply(invalid_chars)]
fn does_not_start_with_invalid_word(#[case] invalid_char: char) {
    let word = format!("aWord{invalid_char}ñ");
    let game = Hangman::start(&word, 1);

    assert!(game.is_err());
}

#[test]
fn does_not_start_with_no_allowed_failures() {
    let game = Hangman::start("aWordñ", 0);

    assert!(game.is_err());
}

#[test]
fn starts_with_valid_input() {
    let game = Hangman::start("aWordñ", 1);

    assert!(game.is_ok());
}
