use hangman::alphabetic_char::AlphabeticChar;
use hangman::errors::CharError;
use rstest::rstest;

#[rstest]
#[case('3')]
#[case(' ')]
#[case('!')]
#[case('#')]
#[case('.')]
#[case('-')]
fn does_not_build_from_invalid_char(#[case] invalid_char: char) {
    let secret_char = AlphabeticChar::from(invalid_char);

    assert!(secret_char.is_err_and(|e| matches!(e, CharError::NonAlphabeticChar)));
}

#[test]
fn can_tell_if_matches() {
    let secret_char = AlphabeticChar::from('a').unwrap();

    assert_eq!(secret_char, AlphabeticChar::from('a').unwrap());
}

#[test]
fn can_tell_if_doesnt_match() {
    let secret_char = AlphabeticChar::from('a').unwrap();

    assert_ne!(secret_char, AlphabeticChar::from('b').unwrap());
}

#[test]
fn handles_non_case_convertible_chars() {
    let secret_char = AlphabeticChar::from('ß').unwrap();

    assert_eq!(secret_char, AlphabeticChar::from('ß').unwrap());
}
