use hangman::errors::SecretCharError;
use hangman::secret_char::SecretChar;
use rstest::rstest;

#[rstest]
#[case('3')]
#[case(' ')]
#[case('!')]
#[case('#')]
#[case('.')]
#[case('-')]
fn does_not_build_from_invalid_char(#[case] invalid_char: char) {
    let secret_char = SecretChar::from(invalid_char);

    assert!(secret_char.is_err_and(|e| matches!(e, SecretCharError::NonAlphabeticChar)));
}

#[test]
fn initially_displays_char_as_hidden() {
    let secret_char = SecretChar::from('a').unwrap();

    assert_eq!(secret_char.display(), '_');
}

#[test]
fn displays_revealed_char_as_uppercase() {
    let mut secret_char = SecretChar::from('a').unwrap();

    secret_char.reveal();

    assert_eq!(secret_char.display(), 'A');
}

#[test]
fn displays_non_case_convertible_char_as_is() {
    let mut secret_char = SecretChar::from('ß').unwrap();

    secret_char.reveal();

    assert_eq!(secret_char.display(), 'ß');
}

#[test]
fn can_tell_if_not_guessed() {
    let secret_char = SecretChar::from('a').unwrap();

    assert!(!secret_char.is_guessed());
}

#[test]
fn can_tell_if_guessed() {
    let mut secret_char = SecretChar::from('a').unwrap();
    secret_char.reveal();

    assert!(secret_char.is_guessed());
}

#[test]
fn can_tell_if_matches() {
    let secret_char = SecretChar::from('a').unwrap();

    assert!(secret_char.matches('a'));
}

#[test]
fn can_tell_if_doesnt_match() {
    let secret_char = SecretChar::from('a').unwrap();

    assert!(!secret_char.matches('b'));
}
