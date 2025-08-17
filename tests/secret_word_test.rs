use hangman::chars::alphabetic::AlphabeticChar;
use hangman::errors::SecretWordError;
use hangman::secret_word::SecretWord;
use rstest::rstest;
use rstest_reuse::apply;

mod helpers;
use helpers::invalid_chars;

#[test]
fn does_not_build_without_word() {
    let secret_word = SecretWord::from("");

    assert!(secret_word.is_err_and(|e| matches!(e, SecretWordError::EmptySecretWord)));
}

#[apply(invalid_chars)]
fn does_not_build_with_invalid_chars_in_word(#[case] invalid_char: char) {
    let invalid_word = format!("a{invalid_char}Word");

    let secret_word = SecretWord::from(&invalid_word);

    assert!(secret_word.is_err_and(|e| matches!(e, SecretWordError::NonAlphabeticCharacters)));
}

#[test]
fn initially_displays_all_chars_as_hidden() {
    let secret_word = SecretWord::from("abc").unwrap();

    assert_eq!(secret_word.to_string(), "___");
}

#[test]
fn can_tell_if_word_contains_char() {
    let secret_word = SecretWord::from("abc").unwrap();
    let contained_char = AlphabeticChar::from('a').unwrap();

    assert!(secret_word.contains(&contained_char));
}

#[test]
fn can_tell_if_word_does_not_contain_char() {
    let secret_word = SecretWord::from("abc").unwrap();
    let not_contained_char = &AlphabeticChar::from('x').unwrap();

    assert!(!secret_word.contains(not_contained_char));
}

#[test]
fn shows_only_revealed_chars() {
    let mut secret_word = SecretWord::from("abc").unwrap();
    secret_word.reveal_char(&AlphabeticChar::from('a').unwrap());
    secret_word.reveal_char(&AlphabeticChar::from('c').unwrap());

    assert_eq!(secret_word.to_string(), "A_C");
}

#[test]
fn shows_all_instances_of_revealed_char() {
    let mut secret_word = SecretWord::from("aba").unwrap();
    secret_word.reveal_char(&AlphabeticChar::from('a').unwrap());

    assert_eq!(secret_word.to_string(), "A_A");
}

#[test]
fn does_not_reveal_char_not_in_word() {
    let mut secret_word = SecretWord::from("abc").unwrap();
    secret_word.reveal_char(&AlphabeticChar::from('x').unwrap());

    assert_eq!(secret_word.to_string(), "___");
}

#[test]
fn reveals_whole_word() {
    let mut secret_word = SecretWord::from("abc").unwrap();
    secret_word.reveal_word();

    assert_eq!(secret_word.to_string(), "ABC");
}

#[test]
fn can_tell_if_all_chars_are_revealed() {
    let mut secret_word = SecretWord::from("abc").unwrap();
    secret_word.reveal_char(&AlphabeticChar::from('a').unwrap());
    secret_word.reveal_char(&AlphabeticChar::from('b').unwrap());
    secret_word.reveal_char(&AlphabeticChar::from('c').unwrap());

    assert!(secret_word.is_revealed());
}

#[test]
fn can_tell_if_not_all_chars_are_revealed() {
    let mut secret_word = SecretWord::from("abc").unwrap();
    secret_word.reveal_char(&AlphabeticChar::from('a').unwrap());
    secret_word.reveal_char(&AlphabeticChar::from('b').unwrap());

    assert!(!secret_word.is_revealed());
}
