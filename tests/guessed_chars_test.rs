mod helpers;

use hangman::guessed_chars::GuessedChars;
use helpers::alphabetic_char_from;

#[test]
fn does_not_build_with_too_little_failures() {
    let guessed_chars = GuessedChars::allowed_failures(0);

    assert!(guessed_chars.is_err());
}

#[test]
pub fn can_add_correct_guess() {
    let mut guessed_chars = GuessedChars::allowed_failures(1).unwrap();

    guessed_chars.add_correct(alphabetic_char_from('a'));

    assert!(guessed_chars
        .correct_guesses()
        .contains(&alphabetic_char_from('a')));
}

#[test]
pub fn can_add_incorrect_guess() {
    let mut guessed_chars = GuessedChars::allowed_failures(1).unwrap();

    guessed_chars.add_failed(alphabetic_char_from('a'));

    assert!(guessed_chars
        .failed_guesses()
        .contains(&alphabetic_char_from('a')));
}

#[test]
fn failures_are_counted() {
    let limit = 3;
    let mut failures = GuessedChars::allowed_failures(limit).unwrap();

    failures.add_failed(alphabetic_char_from('a'));

    assert_eq!(failures.remaining(), limit - 1);
}

#[test]
pub fn can_tell_if_already_guessed_correctly() {
    let mut guessed_chars = GuessedChars::allowed_failures(1).unwrap();

    guessed_chars.add_correct(alphabetic_char_from('a'));

    assert!(guessed_chars.already_guessed(&alphabetic_char_from('a')));
}

#[test]
pub fn can_tell_if_already_guessed_incorrectly() {
    let mut guessed_chars = GuessedChars::allowed_failures(1).unwrap();

    guessed_chars.add_failed(alphabetic_char_from('a'));

    assert!(guessed_chars.already_guessed(&alphabetic_char_from('a')));
}

#[test]
fn no_failures_left() {
    let mut guessed_chars = GuessedChars::allowed_failures(1).unwrap();

    guessed_chars.add_failed(alphabetic_char_from('a'));

    assert!(guessed_chars.no_failures_available());
}

#[test]
fn some_failures_left() {
    let mut guessed_chars = GuessedChars::allowed_failures(2).unwrap();

    guessed_chars.add_failed(alphabetic_char_from('a'));

    assert!(!guessed_chars.no_failures_available());
}
