use hangman::errors::InitError;
use hangman::hangman::{GuessResult, Hangman};
use hangman::state::GameState;
use rstest::rstest;
use rstest_reuse::{apply, template};

const VALID_MAX_FAILED_GUESSES: usize = 1;
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

#[test]
fn does_not_init_with_invalid_max_failed_guesses() {
    let game = Hangman::init(VALID_WORD, 0);

    assert!(game.is_err_and(|e| matches!(e, InitError::NotEnoughGuesses)));
}

#[test]
fn does_not_init_without_word() {
    let game = Hangman::init("", VALID_MAX_FAILED_GUESSES);

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
    let game = Hangman::init(&invalid_word, VALID_MAX_FAILED_GUESSES);

    assert!(game.is_err_and(|e| matches!(e, InitError::NonAlphabeticCharacters)));
}

#[apply(invalid_chars)]
fn does_not_accept_invalid_chars_when_guessing(#[case] invalid_char: char) {
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

#[test]
fn game_stops_when_max_failed_guesses_are_reached() {
    let mut game = Hangman::init("abc", 1).unwrap();

    game.guess('x');
    let guess_result = game.guess('z');

    assert!(matches!(guess_result, GuessResult::GameNotInProgress));
}

#[test]
fn game_stops_when_all_letters_are_guessed() {
    let mut game = Hangman::init("a", 1).unwrap();

    game.guess('a');
    let guess_result = game.guess('b');

    assert!(matches!(guess_result, GuessResult::GameNotInProgress));
}

#[test]
fn game_is_lost_when_max_failed_guesses_are_reached() {
    let mut game = Hangman::init("abc", 1).unwrap();

    game.guess('x');

    assert!(matches!(game.state(), GameState::Lost));
}

#[test]
fn game_is_won_when_all_letters_are_guessed() {
    let mut game = Hangman::init("a", 1).unwrap();

    game.guess('a');

    assert!(matches!(game.state(), GameState::Won));
}
