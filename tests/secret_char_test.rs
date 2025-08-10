use hangman::alphabetic_char::AlphabeticChar;
use hangman::secret_char::SecretChar;

#[test]
fn initially_displays_char_as_hidden() {
    let secret_char = SecretChar::from(AlphabeticChar::from('a').unwrap());

    assert_eq!(secret_char.to_string(), "_");
}

#[test]
fn displays_revealed_char_as_uppercase() {
    let mut secret_char = SecretChar::from(AlphabeticChar::from('a').unwrap());

    secret_char.reveal();

    assert_eq!(secret_char.to_string(), "A");
}

#[test]
fn displays_non_case_convertible_char_as_is() {
    let mut secret_char = SecretChar::from(AlphabeticChar::from('ß').unwrap());

    secret_char.reveal();

    assert_eq!(secret_char.to_string(), "ß");
}

#[test]
fn can_tell_if_not_guessed() {
    let secret_char = SecretChar::from(AlphabeticChar::from('a').unwrap());

    assert!(!secret_char.is_guessed());
}

#[test]
fn can_tell_if_guessed() {
    let mut secret_char = SecretChar::from(AlphabeticChar::from('a').unwrap());
    secret_char.reveal();

    assert!(secret_char.is_guessed());
}

#[test]
fn matches_when_underlying_char_matches() {
    let char = AlphabeticChar::from('a').unwrap();
    let secret_char = SecretChar::from(char);

    let the_same_char = AlphabeticChar::from('a').unwrap();
    assert!(secret_char.matches(&the_same_char));
}

#[test]
fn does_not_match_when_underlying_char_does_not_match() {
    let char = AlphabeticChar::from('a').unwrap();
    let secret_char = SecretChar::from(char);

    let different_char = AlphabeticChar::from('b').unwrap();
    assert!(!secret_char.matches(&different_char));
}
