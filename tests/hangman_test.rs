use hangman::hangman::Hangman;
use hangman::results::GuessResult;
use hangman::states::GameState;
use rstest::rstest;

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

#[rstest]
#[case("3")]
#[case(" ")]
#[case("!")]
#[case("#")]
#[case(".")]
#[case("-")]
fn does_not_accept_invalid_guesses(#[case] invalid_char: char) {
    let game = Hangman::start(VALID_WORD, VALID_ALLOWED_FAILURES);

    let guess_result = game.unwrap().guess(invalid_char);

    assert!(matches!(guess_result, GuessResult::InvalidCharacter));
}

#[test]
fn succeeds_when_guessing_correctly() {
    let game = Hangman::start("Abc", 1);

    let guess_result = game.unwrap().guess('a');

    assert!(matches!(guess_result, GuessResult::Correct));
}

#[test]
fn warns_of_duplicate_when_repeating_a_correct_guess() {
    let mut game = Hangman::start("abc", 1).unwrap();

    game.guess('a');
    let second_guess_result = game.guess('a');

    assert!(matches!(second_guess_result, GuessResult::Duplicate));
}

#[test]
fn warns_of_duplicate_when_repeating_an_incorrect_guess() {
    let mut game = Hangman::start("abc", 2).unwrap();

    game.guess('x');
    let second_guess_result = game.guess('x');

    assert!(matches!(second_guess_result, GuessResult::Duplicate));
}

#[test]
fn fails_when_guessing_incorrectly() {
    let game = Hangman::start("abc", 1);

    let guess_result = game.unwrap().guess('z');

    assert!(matches!(guess_result, GuessResult::Incorrect));
}

#[test]
fn game_stops_when_allowed_failures_are_surpassed() {
    let mut game = Hangman::start("abc", 1).unwrap();

    game.guess('x');
    let guess_result = game.guess('z');

    assert!(matches!(guess_result, GuessResult::GameNotInProgress));
}

#[test]
fn game_stops_when_word_is_guessed() {
    let mut game = Hangman::start("a", 1).unwrap();

    game.guess('a');
    let guess_result = game.guess('b');

    assert!(matches!(guess_result, GuessResult::GameNotInProgress));
}

#[test]
fn game_is_lost_when_allowed_failures_are_surpassed() {
    let mut game = Hangman::start("abc", 1).unwrap();

    game.guess('x');

    assert!(matches!(game.state(), GameState::Lost));
}

#[test]
fn game_is_won_when_word_is_guessed() {
    let mut game = Hangman::start("a", 1).unwrap();

    game.guess('a');

    assert!(matches!(game.state(), GameState::Won));
}

#[test]
fn shows_already_guessed_chars() {
    let mut game = Hangman::start("abc", 2).unwrap();
    game.guess('a');
    game.guess('x');

    let already_guessed = game.already_guessed();

    assert_eq!(
        already_guessed,
        "\n\tCorrect guesses: A\n\tIncorrect guesses: X"
    );
}
