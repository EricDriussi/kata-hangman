use hangman::hangman::{GameState, Hangman, Running};
use hangman::results::GuessResult;
use rstest::rstest;
use rstest_reuse::apply;

mod helpers;
use helpers::invalid_chars;

const VALID_ALLOWED_FAILURES: isize = 1;
const VALID_WORD: &str = "aWordñ";

#[test]
fn starts_with_valid_word_and_limit() {
    let game = Hangman::<Running>::start(VALID_WORD, VALID_ALLOWED_FAILURES);

    assert!(game.is_ok());
}

#[apply(invalid_chars)]
fn does_not_accept_invalid_guesses(#[case] invalid_char: char) {
    let game = Hangman::<Running>::start(VALID_WORD, VALID_ALLOWED_FAILURES).unwrap();

    let (guess_result, _) = game.guess(invalid_char);

    assert!(matches!(guess_result, GuessResult::Invalid));
}

#[test]
fn succeeds_when_guessing_correctly() {
    let game = Hangman::<Running>::start("Abc", 1).unwrap();

    let (guess_result, _) = game.guess('a');

    assert!(matches!(guess_result, GuessResult::Correct));
}

#[test]
fn warns_of_duplicate_when_repeating_a_correct_guess() {
    let game = Hangman::<Running>::start("abc", 1).unwrap();

    let (_, game_state) = game.guess('a');
    let GameState::InProgress(game) = game_state else {
        panic!("Expected game to be InProgress");
    };

    let (guess_result, _) = game.guess('a');
    assert!(matches!(guess_result, GuessResult::Duplicate));
}
#[test]
fn warns_of_duplicate_when_repeating_an_incorrect_guess() {
    let game = Hangman::<Running>::start("abc", 2).unwrap();

    let (_, game_state) = game.guess('x');
    let GameState::InProgress(game) = game_state else {
        panic!("Expected game to be InProgress");
    };

    let (guess_result, _) = game.guess('x');
    assert!(matches!(guess_result, GuessResult::Duplicate));
}

#[test]
fn fails_when_guessing_incorrectly() {
    let game = Hangman::<Running>::start("abc", 1).unwrap();

    let (guess_result, _) = game.guess('z');

    assert!(matches!(guess_result, GuessResult::Incorrect));
}

#[test]
fn game_is_lost_when_allowed_failures_are_surpassed() {
    let game = Hangman::<Running>::start("abc", 1).unwrap();

    let (_, game_state) = game.guess('x');

    assert!(matches!(game_state, GameState::Lost(_)));
}

#[test]
fn game_is_won_when_word_is_guessed() {
    let game = Hangman::<Running>::start("a", 1).unwrap();

    let (_, game_state) = game.guess('a');

    assert!(matches!(game_state, GameState::Won(_)));
}

#[test]
fn shows_already_guessed_chars() {
    let game = Hangman::<Running>::start("abc", 2).unwrap();

    let (_, game_state) = game.guess('a');
    let GameState::InProgress(game) = game_state else {
        panic!("Expected game to be InProgress");
    };

    let (_, game_state) = game.guess('x');
    let GameState::InProgress(game) = game_state else {
        panic!("Expected game to be InProgress");
    };

    assert!(game
        .to_string()
        .contains("\n\tCorrect guesses: A\n\tIncorrect guesses: X"));
}
