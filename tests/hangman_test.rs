use hangman::errors::InitError;
use hangman::hangman::{GuessResult, Hangman};
use hangman::state::GameState;
use rstest::rstest;

const VALID_MAX_FAILED_GUESSES: isize = 1;
const VALID_WORD: &str = "aWordñ";

#[test]
fn starts_with_valid_word_and_limit() {
    let game = Hangman::init(VALID_WORD, VALID_MAX_FAILED_GUESSES);

    assert!(game.is_ok());
}

#[test]
fn starts_with_in_progress_state() {
    let game = Hangman::init(VALID_WORD, VALID_MAX_FAILED_GUESSES);

    assert!(game.is_ok_and(|g| g.state() == GameState::InProgress));
}

#[rstest]
#[case(0)]
#[case(-1)]
#[case(-2)]
fn does_not_init_with_invalid_max_failed_guesses(#[case] invalid_max_failed_guesses: isize) {
    let game = Hangman::init(VALID_WORD, invalid_max_failed_guesses);

    assert!(game.is_err_and(|e| matches!(e, InitError::NotEnoughGuesses)));
}

#[test]
fn does_not_init_without_word() {
    let game = Hangman::init("", VALID_MAX_FAILED_GUESSES);

    assert!(game.is_err_and(|e| matches!(e, InitError::EmptySecretWord)));
}

#[rstest]
#[case("3")]
#[case(" ")]
#[case("!")]
#[case("#")]
#[case(".")]
#[case("-")]
fn does_not_init_with_invalid_characters_in_word(#[case] invalid_char: &str) {
    let invalid_word = format!("a{invalid_char}Word");
    let game = Hangman::init(&invalid_word, VALID_MAX_FAILED_GUESSES);

    assert!(game.is_err_and(|e| matches!(e, InitError::NonAlphabeticCharacters)));
}

#[rstest]
#[case("3")]
#[case(" ")]
#[case("!")]
#[case("#")]
#[case(".")]
#[case("-")]
fn does_not_accept_invalid_characters_when_guessing(#[case] invalid_char: char) {
    let game = Hangman::init(VALID_WORD, VALID_MAX_FAILED_GUESSES);

    let guess_result = game.unwrap().guess(invalid_char);

    assert!(matches!(guess_result, GuessResult::InvalidCharacter));
}

#[test]
fn succeeds_when_guessing_a_matching_char() {
    let game = Hangman::init("Abc", 1);

    let guess_result = game.unwrap().guess('a');

    assert!(matches!(guess_result, GuessResult::Correct));
}

#[test]
fn fails_when_when_guessing_a_previously_guessed_char() {
    let mut game = Hangman::init("abc", 1).unwrap();

    game.guess('a');
    let second_guess_result = game.guess('a');
    assert!(matches!(second_guess_result, GuessResult::Duplicate));
}

#[test]
fn fails_when_guessing_a_non_matching_char() {
    let game = Hangman::init("abc", 1);

    let guess_result = game.unwrap().guess('z');

    assert!(matches!(guess_result, GuessResult::Incorrect));
}
