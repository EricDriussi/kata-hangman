use hangman::char::Char;
use hangman::guessed_char::GuessedChar;

#[test]
fn can_tell_if_was_correct() {
    let guessed_char = GuessedChar::correct(Char::from('a').unwrap());

    assert!(guessed_char.was_correct());
}

#[test]
fn can_tell_if_was_incorrect() {
    let guessed_char = GuessedChar::incorrect(Char::from('a').unwrap());

    assert!(guessed_char.was_incorrect());
}

#[test]
fn matches_when_underlying_char_matches() {
    let char = Char::from('a').unwrap();
    let guessed_char = GuessedChar::correct(char);

    let the_same_char = Char::from('a').unwrap();
    assert!(guessed_char.matches(&the_same_char));
}

#[test]
fn does_not_match_when_underlying_char_does_not_match() {
    let char = Char::from('a').unwrap();
    let guessed_char = GuessedChar::correct(char);

    let different_char = Char::from('b').unwrap();
    assert!(!guessed_char.matches(&different_char));
}
