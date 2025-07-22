use hangman::errors::InitError;
use hangman::hangman::{GuessResult, Hangman};
use hangman::state::GameState;
use rstest::rstest;
use rstest_reuse::{apply, template};

const VALID_ALLOWED_FAILURES: usize = 1;
const VALID_WORD: &str = "aWordñ";

#[test]
fn starts_with_valid_word_and_limit() {
    let game = Hangman::init(VALID_WORD, VALID_ALLOWED_FAILURES);

    assert!(game.is_ok());
}

#[test]
fn starts_with_in_progress_state() {
    let game = Hangman::init(VALID_WORD, VALID_ALLOWED_FAILURES);

    assert!(game.is_ok_and(|g| g.state() == GameState::InProgress));
}

#[test]
fn does_not_init_with_invalid_allowed_failures() {
    let game = Hangman::init(VALID_WORD, 0);

    assert!(game.is_err_and(|e| matches!(e, InitError::NotEnoughGuesses)));
}

#[test]
fn does_not_init_without_word() {
    let game = Hangman::init("", VALID_ALLOWED_FAILURES);

    assert!(game.is_err_and(|e| matches!(e, InitError::EmptySecretWord)));
}

#[template]
#[rstest]
#[case("3")]
#[case(" ")]
#[case("!")]
#[case("#")]
#[case(".")]
#[case("-")]
fn invalid_chars(#[case] a: char) {}

#[apply(invalid_chars)]
fn does_not_init_with_invalid_chars_in_word(#[case] invalid_char: char) {
    let invalid_word = format!("a{invalid_char}Word");
    let game = Hangman::init(&invalid_word, VALID_ALLOWED_FAILURES);

    assert!(game.is_err_and(|e| matches!(e, InitError::NonAlphabeticCharacters)));
}

#[apply(invalid_chars)]
fn does_not_accept_invalid_chars_when_guessing(#[case] invalid_char: char) {
    let game = Hangman::init(VALID_WORD, VALID_ALLOWED_FAILURES);

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
fn warns_of_duplicate_when_when_guessing_a_previously_guessed_char() {
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

#[test]
fn game_stops_when_allowed_failures_are_reached() {
    let mut game = Hangman::init("abc", 1).unwrap();

    game.guess('x');
    let guess_result = game.guess('z');

    assert!(matches!(guess_result, GuessResult::GameNotInProgress));
}

#[test]
fn game_stops_when_all_chars_are_guessed() {
    let mut game = Hangman::init("a", 1).unwrap();

    game.guess('a');
    let guess_result = game.guess('b');

    assert!(matches!(guess_result, GuessResult::GameNotInProgress));
}

#[test]
fn game_is_lost_when_allowed_failures_are_reached() {
    let mut game = Hangman::init("abc", 1).unwrap();

    game.guess('x');

    assert!(matches!(game.state(), GameState::Lost));
}

#[test]
fn game_is_won_when_all_chars_are_guessed() {
    let mut game = Hangman::init("a", 1).unwrap();

    game.guess('a');

    assert!(matches!(game.state(), GameState::Won));
}

#[test]
fn word_is_initially_displayed_hiding_all_chars() {
    let game = Hangman::init("abc", VALID_ALLOWED_FAILURES).unwrap();

    let word: String = game.display_word();

    assert_eq!(word, "___");
}

#[test]
fn word_is_displayed_showing_guessed_char() {
    let mut game = Hangman::init("abc", VALID_ALLOWED_FAILURES).unwrap();
    game.guess('a');
    game.guess('c');

    let word: String = game.display_word();

    assert_eq!(word, "A_C");
}
