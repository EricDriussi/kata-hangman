use hangman::alphabetic_char::AlphabeticChar;
use rstest_reuse::template;

// TODO: why do I need this allow?
#[allow(dead_code)]
pub fn alphabetic_char_from(c: char) -> AlphabeticChar {
    AlphabeticChar::from(c).unwrap()
}

#[template]
#[export]
#[rstest]
#[case('3')]
#[case(' ')]
#[case('!')]
#[case('#')]
#[case('.')]
#[case('-')]
pub fn invalid_chars(#[case] invalid_char: char) {}
