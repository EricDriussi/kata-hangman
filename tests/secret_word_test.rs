use hangman::errors::SecretWordError;
use hangman::secret_word::SecretWord;
use rstest::rstest;

#[test]
fn does_not_build_without_word() {
    let word = SecretWord::from("");

    assert!(word.is_err_and(|e| matches!(e, SecretWordError::EmptySecretWord)));
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
    let word = SecretWord::from(&invalid_word);

    assert!(word.is_err_and(|e| matches!(e, SecretWordError::NonAlphabeticCharacters)));
}

#[test]
fn initially_displays_all_chars_as_hidden() {
    let word = SecretWord::from("abc");

    let word: String = word.unwrap().display();

    assert_eq!(word, "___");
}

#[test]
fn can_tell_if_word_contains_char() {
    let word = SecretWord::from("abc");

    assert!(word.unwrap().contains('a'));
}

#[test]
fn can_tell_if_word_does_not_contain_char() {
    let word = SecretWord::from("abc");

    assert!(!word.unwrap().contains('x'));
}

#[test]
fn displays_revealed_chars() {
    let mut word = SecretWord::from("abc").unwrap();
    word.reveal('a');
    word.reveal('c');

    let word: String = word.display();

    assert_eq!(word, "A_C");
}

#[test]
fn displays_all_instances_of_revealed_char() {
    let mut word = SecretWord::from("aba").unwrap();
    word.reveal('a');

    let word: String = word.display();

    assert_eq!(word, "A_A");
}

#[test]
fn does_not_reveal_char_not_in_word() {
    let mut word = SecretWord::from("abc").unwrap();
    word.reveal('x');

    let word: String = word.display();

    assert_eq!(word, "___");
}

#[test]
fn can_tell_if_all_chars_are_revealed() {
    let mut word = SecretWord::from("abc").unwrap();
    word.reveal('a');
    word.reveal('b');
    word.reveal('c');

    assert!(word.is_revealed());
}
#[test]
fn can_tell_if_not_all_chars_are_revealed() {
    let mut word = SecretWord::from("abc").unwrap();
    word.reveal('a');
    word.reveal('b');

    assert!(!word.is_revealed());
}
