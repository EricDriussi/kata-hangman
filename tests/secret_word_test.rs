use hangman::errors::SecretWordError;
use hangman::secret_word::SecretWord;
use rstest::rstest;

#[test]
fn does_not_build_without_word() {
    let secret_word = SecretWord::from("");

    assert!(secret_word.is_err_and(|e| matches!(e, SecretWordError::EmptySecretWord)));
}

#[rstest]
#[case("3")]
#[case(" ")]
#[case("!")]
#[case("#")]
#[case(".")]
#[case("-")]
fn does_not_build_with_invalid_chars_in_word(#[case] invalid_char: char) {
    let invalid_word = format!("a{invalid_char}Word");

    let secret_word = SecretWord::from(&invalid_word);

    assert!(secret_word.is_err_and(|e| matches!(e, SecretWordError::NonAlphabeticCharacters)));
}

#[test]
fn initially_displays_all_chars_as_hidden() {
    let secret_word = SecretWord::from("abc").unwrap();

    assert_eq!(secret_word.display(), "___");
}

#[test]
fn can_tell_if_word_contains_char() {
    let secret_word = SecretWord::from("abc").unwrap();

    assert!(secret_word.contains('a'));
}

#[test]
fn can_tell_if_word_does_not_contain_char() {
    let secret_word = SecretWord::from("abc").unwrap();

    assert!(!secret_word.contains('x'));
}

#[test]
fn displays_revealed_chars() {
    let mut secret_word = SecretWord::from("abc").unwrap();

    secret_word.reveal('a');
    secret_word.reveal('c');

    assert_eq!(secret_word.display(), "A_C");
}

#[test]
fn displays_all_instances_of_revealed_char() {
    let mut secret_word = SecretWord::from("aba").unwrap();

    secret_word.reveal('a');

    assert_eq!(secret_word.display(), "A_A");
}

#[test]
fn does_not_reveal_char_not_in_word() {
    let mut secret_word = SecretWord::from("abc").unwrap();

    secret_word.reveal('x');

    assert_eq!(secret_word.display(), "___");
}

#[test]
fn can_tell_if_all_chars_are_revealed() {
    let mut secret_word = SecretWord::from("abc").unwrap();
    secret_word.reveal('a');
    secret_word.reveal('b');
    secret_word.reveal('c');

    assert!(secret_word.is_revealed());
}
#[test]
fn can_tell_if_not_all_chars_are_revealed() {
    let mut secret_word = SecretWord::from("abc").unwrap();
    secret_word.reveal('a');
    secret_word.reveal('b');

    assert!(!secret_word.is_revealed());
}
