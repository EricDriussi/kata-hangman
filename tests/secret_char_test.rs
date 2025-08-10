mod helpers;
use hangman::secret_char::SecretChar;
use helpers::alphabetic_char_from;

#[test]
fn initially_displays_char_as_hidden() {
    let secret_char = SecretChar::from(alphabetic_char_from('a'));

    assert_eq!(secret_char.to_string(), "_");
}

#[test]
fn displays_revealed_char_as_uppercase() {
    let mut secret_char = SecretChar::from(alphabetic_char_from('a'));

    secret_char.reveal();

    assert_eq!(secret_char.to_string(), "A");
}

#[test]
fn displays_non_case_convertible_char_as_is() {
    let mut secret_char = SecretChar::from(alphabetic_char_from('ß'));

    secret_char.reveal();

    assert_eq!(secret_char.to_string(), "ß");
}

#[test]
fn can_tell_if_not_guessed() {
    let secret_char = SecretChar::from(alphabetic_char_from('a'));

    assert!(!secret_char.is_guessed());
}

#[test]
fn can_tell_if_guessed() {
    let mut secret_char = SecretChar::from(alphabetic_char_from('a'));

    secret_char.reveal();

    assert!(secret_char.is_guessed());
}

#[test]
fn is_eq_based_on_underlying_char() {
    let secret_char = SecretChar::from(alphabetic_char_from('a'));

    let the_same_char = alphabetic_char_from('a');
    assert_eq!(secret_char, the_same_char);
}

#[test]
fn is_not_eq_based_on_underlying_char() {
    let secret_char = SecretChar::from(alphabetic_char_from('a'));

    let different_char = alphabetic_char_from('b');
    assert_ne!(secret_char, different_char);
}
