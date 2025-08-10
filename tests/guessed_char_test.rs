use hangman::alphabetic_char::AlphabeticChar;
use hangman::guessed_char::GuessedChar;

#[test]
fn can_tell_if_was_correct() {
    let guessed_char = GuessedChar::correct(AlphabeticChar::from('a').unwrap());

    assert!(guessed_char.was_correct());
}

#[test]
fn can_tell_if_was_incorrect() {
    let guessed_char = GuessedChar::incorrect(AlphabeticChar::from('a').unwrap());

    assert!(guessed_char.was_incorrect());
}

#[test]
fn matches_when_underlying_char_matches() {
    let alphabetic_char = AlphabeticChar::from('a').unwrap();
    let guessed_char = GuessedChar::correct(alphabetic_char);

    let the_same_char = AlphabeticChar::from('a').unwrap();
    assert_eq!(guessed_char, the_same_char);
}

#[test]
fn does_not_match_when_underlying_char_does_not_match() {
    let alphabetic_char = AlphabeticChar::from('a').unwrap();
    let guessed_char = GuessedChar::correct(alphabetic_char);

    let different_char = AlphabeticChar::from('b').unwrap();
    assert_ne!(guessed_char, different_char);
}
