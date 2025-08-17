use crate::helpers::invalid_chars;
use hangman::game_state::GameState;
use hangman::hangman::running_hangman::RunningHangman;
use hangman::results::GuessResult;
use rstest::rstest;
use rstest_reuse::apply;

const VALID_ALLOWED_FAILURES: isize = 1;
const VALID_WORD: &str = "aWordñ";

#[test]
fn starts_with_valid_word_and_limit() {
    let game = RunningHangman::start(VALID_WORD, VALID_ALLOWED_FAILURES);

    assert!(game.is_ok());
}

#[apply(invalid_chars)]
fn does_not_accept_invalid_guesses(#[case] invalid_char: char) {
    let game = RunningHangman::start(VALID_WORD, VALID_ALLOWED_FAILURES).unwrap();

    let (guess_result, _) = game.guess(invalid_char);

    assert!(matches!(guess_result, GuessResult::Invalid));
}

#[test]
fn succeeds_when_guessing_correctly() {
    let game = RunningHangman::start("Abc", 1).unwrap();

    let (guess_result, _) = game.guess('a');

    assert!(matches!(guess_result, GuessResult::Correct));
}

#[test]
fn warns_of_duplicate_when_repeating_a_correct_guess() {
    let game = RunningHangman::start("abc", 1).unwrap();

    let (_, game_state) = game.guess('a');
    let GameState::InProgress(game) = game_state else {
        panic!("Expected game to be InProgress");
    };

    let (guess_result, _) = game.guess('a');
    assert!(matches!(guess_result, GuessResult::Duplicate));
}
#[test]
fn warns_of_duplicate_when_repeating_an_incorrect_guess() {
    let game = RunningHangman::start("abc", 2).unwrap();

    let (_, game_state) = game.guess('x');
    let GameState::InProgress(game) = game_state else {
        panic!("Expected game to be InProgress");
    };

    let (guess_result, _) = game.guess('x');
    assert!(matches!(guess_result, GuessResult::Duplicate));
}

#[test]
fn fails_when_guessing_incorrectly() {
    let game = RunningHangman::start("abc", 1).unwrap();

    let (guess_result, _) = game.guess('x');

    assert!(matches!(guess_result, GuessResult::Incorrect));
}

#[test]
fn game_is_lost_when_allowed_failures_are_surpassed() {
    let game = RunningHangman::start("abc", 1).unwrap();

    let (_, game_state) = game.guess('x');

    assert!(matches!(game_state, GameState::Lost(_)));
}

#[test]
fn game_is_won_when_word_is_guessed() {
    let game = RunningHangman::start("a", 1).unwrap();

    let (_, game_state) = game.guess('a');

    assert!(matches!(game_state, GameState::Won(_)));
}

#[test]
fn shows_secret_word() {
    let game = RunningHangman::start("abc", 1).unwrap();

    let correct_char = 'a';
    let (_, game_state) = game.guess(correct_char);
    let GameState::InProgress(game) = game_state else {
        panic!("Expected game to be InProgress");
    };

    assert!(game.to_string().contains(&format!(
        "Secret word: {}__\n",
        correct_char.to_ascii_uppercase()
    )));
}

#[test]
fn shows_remaining_failures() {
    let allowed_failures = 2;
    let game = RunningHangman::start("abc", allowed_failures).unwrap();

    let (_, game_state) = game.guess('x');
    let GameState::InProgress(game) = game_state else {
        panic!("Expected game to be InProgress");
    };

    assert!(game
        .to_string()
        .contains(&format!("Remaining failures: {}\n", allowed_failures - 1)));
}

#[test]
fn shows_already_guessed_chars() {
    let game = RunningHangman::start("abc", 2).unwrap();

    let correct_char = 'a';
    let (_, game_state) = game.guess(correct_char);
    let GameState::InProgress(game) = game_state else {
        panic!("Expected game to be InProgress");
    };

    let incorrect_char = 'x';
    let (_, game_state) = game.guess(incorrect_char);
    let GameState::InProgress(game) = game_state else {
        panic!("Expected game to be InProgress");
    };

    assert!(game.to_string().contains(&format!(
        "\n\tCorrect guesses: {}\n\tIncorrect guesses: {}",
        correct_char.to_ascii_uppercase(),
        incorrect_char.to_ascii_uppercase()
    )));
}
