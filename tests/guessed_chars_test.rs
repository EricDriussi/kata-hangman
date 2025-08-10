mod helpers;
use hangman::guessed_chars::GuessedChars;
use helpers::alphabetic_char_from;

#[test]
pub fn can_add_correct_guess() {
    let mut guessed_chars = GuessedChars::none();

    guessed_chars.add_correct(alphabetic_char_from('a'));

    assert!(guessed_chars
        .correct_guesses()
        .contains(&alphabetic_char_from('a')));
}

#[test]
pub fn can_add_incorrect_guess() {
    let mut guessed_chars = GuessedChars::none();

    guessed_chars.add_incorrect(alphabetic_char_from('a'));

    assert!(guessed_chars
        .incorrect_guesses()
        .contains(&alphabetic_char_from('a')));
}

#[test]
pub fn can_tell_if_already_guessed_correctly() {
    let mut guessed_chars = GuessedChars::none();

    guessed_chars.add_correct(alphabetic_char_from('a'));

    assert!(guessed_chars.already_guessed(&alphabetic_char_from('a')));
}

#[test]
pub fn can_tell_if_already_guessed_incorrectly() {
    let mut guessed_chars = GuessedChars::none();

    guessed_chars.add_incorrect(alphabetic_char_from('a'));

    assert!(guessed_chars.already_guessed(&alphabetic_char_from('a')));
}
