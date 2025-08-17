use crate::helpers::alphabetic_char_from;
use hangman::chars::guessed::GuessedChar;

#[test]
fn builds_correct() {
    let alphabetic_char = alphabetic_char_from('a');
    let guessed_char = GuessedChar::correct(alphabetic_char);

    assert!(matches!(guessed_char, GuessedChar::Correct(_)));
    assert_eq!(guessed_char, alphabetic_char_from('a'));
}

#[test]
fn builds_incorrect() {
    let alphabetic_char = alphabetic_char_from('a');
    let guessed_char = GuessedChar::incorrect(alphabetic_char);

    assert!(matches!(guessed_char, GuessedChar::Incorrect(_)));
    assert_eq!(guessed_char, alphabetic_char_from('a'));
}

#[test]
fn is_eq_based_on_underlying_char() {
    let alphabetic_char = alphabetic_char_from('a');
    let guessed_char = GuessedChar::correct(alphabetic_char);

    let the_same_char = alphabetic_char_from('a');
    assert_eq!(guessed_char, the_same_char);
}

#[test]
fn is_not_eq_based_on_underlying_char() {
    let alphabetic_char = alphabetic_char_from('a');
    let guessed_char = GuessedChar::correct(alphabetic_char);

    let different_char = alphabetic_char_from('b');
    assert_ne!(guessed_char, different_char);
}
