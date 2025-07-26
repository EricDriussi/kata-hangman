use hangman::char::Char;
use hangman::secret_char::SecretChar;

#[test]
fn initially_displays_char_as_hidden() {
    let secret_char = SecretChar::from(Char::from('a').unwrap()).unwrap();

    assert_eq!(secret_char.to_string(), "_");
}

#[test]
fn displays_revealed_char_as_uppercase() {
    let mut secret_char = SecretChar::from(Char::from('a').unwrap()).unwrap();

    secret_char.reveal();

    assert_eq!(secret_char.to_string(), "A");
}

#[test]
fn displays_non_case_convertible_char_as_is() {
    let mut secret_char = SecretChar::from(Char::from('ß').unwrap()).unwrap();

    secret_char.reveal();

    assert_eq!(secret_char.to_string(), "ß");
}

#[test]
fn can_tell_if_not_guessed() {
    let secret_char = SecretChar::from(Char::from('a').unwrap()).unwrap();

    assert!(!secret_char.is_guessed());
}

#[test]
fn can_tell_if_guessed() {
    let mut secret_char = SecretChar::from(Char::from('a').unwrap()).unwrap();
    secret_char.reveal();

    assert!(secret_char.is_guessed());
}

#[test]
fn can_tell_if_matches() {
    let secret_char = SecretChar::from(Char::from('a').unwrap()).unwrap();

    assert!(secret_char.matches('a'));
}

#[test]
fn can_tell_if_doesnt_match() {
    let secret_char = SecretChar::from(Char::from('a').unwrap()).unwrap();

    assert!(!secret_char.matches('b'));
}
