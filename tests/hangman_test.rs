use hangman::errors::GuessError;
use hangman::hangman::Hangman;
use hangman::results::GuessResult;
use hangman::states::GameState;
use rstest::rstest;
use rstest_reuse::apply;

mod helpers;
use helpers::invalid_chars;

const VALID_ALLOWED_FAILURES: isize = 1;
const VALID_WORD: &str = "aWordñ";

#[test]
fn starts_with_valid_word_and_limit() {
    let game = Hangman::start(VALID_WORD, VALID_ALLOWED_FAILURES);

    assert!(game.is_ok());
}

#[test]
fn starts_with_in_progress_state() {
    let game = Hangman::start(VALID_WORD, VALID_ALLOWED_FAILURES);

    assert!(game.is_ok_and(|g| g.state() == GameState::InProgress));
}

#[apply(invalid_chars)]
fn does_not_accept_invalid_guesses(#[case] invalid_char: char) {
    let mut game = Hangman::start(VALID_WORD, VALID_ALLOWED_FAILURES).unwrap();

    let guess_result = game.guess(invalid_char);

    assert!(guess_result.is_err_and(|e| matches!(e, GuessError::InvalidCharacter)));
}

#[test]
fn succeeds_when_guessing_correctly() {
    let mut game = Hangman::start("Abc", 1).unwrap();

    let guess_result = game.guess('a').unwrap();

    assert!(matches!(guess_result, GuessResult::Correct));
}

#[test]
fn warns_of_duplicate_when_repeating_a_correct_guess() {
    let mut game = Hangman::start("abc", 1).unwrap();

    game.guess('a').unwrap();
    let duplicate_guess = game.guess('a').unwrap();

    assert!(matches!(duplicate_guess, GuessResult::Duplicate));
}

#[test]
fn warns_of_duplicate_when_repeating_an_incorrect_guess() {
    let mut game = Hangman::start("abc", 2).unwrap();

    game.guess('x').unwrap();
    let second_guess_result = game.guess('x').unwrap();

    assert!(matches!(second_guess_result, GuessResult::Duplicate));
}

#[test]
fn fails_when_guessing_incorrectly() {
    let mut game = Hangman::start("abc", 1).unwrap();

    let guess_result = game.guess('z').unwrap();

    assert!(matches!(guess_result, GuessResult::Incorrect));
}

#[test]
fn game_stops_when_allowed_failures_are_surpassed() {
    let mut game = Hangman::start("abc", 1).unwrap();

    game.guess('x').unwrap();
    let guess_result = game.guess('z');

    assert!(guess_result.is_err_and(|e| matches!(e, GuessError::GameNotInProgress)));
}

#[test]
fn game_stops_when_word_is_guessed() {
    let mut game = Hangman::start("a", 1).unwrap();

    game.guess('a').unwrap();
    let guess_result = game.guess('b');

    assert!(guess_result.is_err_and(|e| matches!(e, GuessError::GameNotInProgress)));
}

#[test]
fn game_is_lost_when_allowed_failures_are_surpassed() {
    let mut game = Hangman::start("abc", 1).unwrap();

    game.guess('x').unwrap();

    assert!(matches!(game.state(), GameState::Lost));
}

#[test]
fn game_is_won_when_word_is_guessed() {
    let mut game = Hangman::start("a", 1).unwrap();

    game.guess('a').unwrap();

    assert!(matches!(game.state(), GameState::Won));
}

#[test]
fn shows_already_guessed_chars() {
    let mut game = Hangman::start("abc", 2).unwrap();
    game.guess('a').unwrap();
    game.guess('x').unwrap();

    let already_guessed = game.already_guessed();

    assert_eq!(
        already_guessed,
        "\n\tCorrect guesses: A\n\tIncorrect guesses: X"
    );
}
