use rstest::rstest;
use hangman::errors::InitError;
use hangman::hangman::Hangman;
use hangman::state::GameState;

const VALID_GUESS_LIMIT: usize = 1;
const VALID_WORD: &str = "aWord";

#[test]
fn starts_with_valid_word_and_limit() {
    let game = Hangman::init(VALID_WORD, VALID_GUESS_LIMIT);

    assert!(game.is_ok());
}

#[test]
fn starts_with_in_progress_state() {
    let game = Hangman::init(VALID_WORD, VALID_GUESS_LIMIT);

    assert!(game.is_ok_and(|g| g.state() == GameState::InProgress));
}

#[test]
fn does_not_init_with_less_than_1_incorrect_guess_limit() {
    let game = Hangman::init(VALID_WORD, 0);

    assert!(game.is_err_and(|e| matches!(e, InitError::NotEnoughGuesses)));
}

#[test]
fn does_not_init_without_secret_word() {
    let game = Hangman::init("", VALID_GUESS_LIMIT);

    assert!(game.is_err_and(|e| matches!(e, InitError::EmptySecretWord)));
}

#[rstest]
#[case("3")]
#[case(" ")]
#[case("!")]
#[case("#")]
#[case(".")]
#[case("-")]
fn does_not_init_with_invalid_characters(#[case] invalid_char: &str) {
    let invalid_word = format!("a{}Word", invalid_char);
    let game = Hangman::init(&invalid_word, VALID_GUESS_LIMIT);

    assert!(game.is_err_and(|e| matches!(e, InitError::NonAlphabeticCharacters)));
}
