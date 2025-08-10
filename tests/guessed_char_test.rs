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
    let char = AlphabeticChar::from('a').unwrap();
    let guessed_char = GuessedChar::correct(char);

    let the_same_char = AlphabeticChar::from('a').unwrap();
    assert!(guessed_char.matches(&the_same_char));
}

#[test]
fn does_not_match_when_underlying_char_does_not_match() {
    let char = AlphabeticChar::from('a').unwrap();
    let guessed_char = GuessedChar::correct(char);

    let different_char = AlphabeticChar::from('b').unwrap();
    assert!(!guessed_char.matches(&different_char));
}
