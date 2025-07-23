use hangman::secret_word::SecretWord;

#[test]
fn initially_displays_all_chars_as_hidden() {
    let word = SecretWord::new("abc");

    let word: String = word.display();

    assert_eq!(word, "___");
}

#[test]
fn can_tell_if_word_contains_char() {
    let word = SecretWord::new("abc");

    assert!(word.contains('a'));
}

#[test]
fn can_tell_if_word_does_not_contain_char() {
    let word = SecretWord::new("abc");

    assert!(!word.contains('x'));
}

#[test]
fn displays_revealed_chars() {
    let mut word = SecretWord::new("abc");
    word.reveal('a');
    word.reveal('c');

    let word: String = word.display();

    assert_eq!(word, "A_C");
}

#[test]
fn displays_all_instances_of_revealed_char() {
    let mut word = SecretWord::new("aba");
    word.reveal('a');

    let word: String = word.display();

    assert_eq!(word, "A_A");
}

#[test]
fn does_not_reveal_char_not_in_word() {
    let mut word = SecretWord::new("abc");
    word.reveal('x');

    let word: String = word.display();

    assert_eq!(word, "___");
}

#[test]
fn can_tell_if_all_chars_are_revealed() {
    let mut word = SecretWord::new("abc");
    word.reveal('a');
    word.reveal('b');
    word.reveal('c');

    assert!(word.is_revealed());
}
#[test]
fn can_tell_if_not_all_chars_are_revealed() {
    let mut word = SecretWord::new("abc");
    word.reveal('a');
    word.reveal('b');

    assert!(!word.is_revealed());
}
