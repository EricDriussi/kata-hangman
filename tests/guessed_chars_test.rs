use hangman::alphabetic_char::AlphabeticChar;
use hangman::guessed_chars::GuessedChars;

#[test]
pub fn can_add_correct_guess() {
    let char = AlphabeticChar::from('a').unwrap();
    let mut guessed_chars = GuessedChars::none();

    guessed_chars.add_correct(char.clone());

    assert!(guessed_chars.correct_guesses().contains(&char));
}

#[test]
pub fn can_add_incorrect_guess() {
    let char = AlphabeticChar::from('a').unwrap();
    let mut guessed_chars = GuessedChars::none();

    guessed_chars.add_incorrect(char.clone());

    assert!(guessed_chars.incorrect_guesses().contains(&char));
}

#[test]
pub fn can_tell_if_already_guessed_correctly() {
    let mut guessed_chars = GuessedChars::none();
    let correct_char = AlphabeticChar::from('a').unwrap();
    guessed_chars.add_correct(correct_char.clone());

    assert!(guessed_chars.already_guessed(&correct_char));
}

#[test]
pub fn can_tell_if_already_guessed_incorrectly() {
    let mut guessed_chars = GuessedChars::none();
    let incorrect_char = AlphabeticChar::from('b').unwrap();
    guessed_chars.add_incorrect(incorrect_char.clone());

    assert!(guessed_chars.already_guessed(&incorrect_char));
}
