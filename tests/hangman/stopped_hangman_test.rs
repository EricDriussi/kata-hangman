use hangman::game_state::GameState;
use hangman::hangman::generic_hangman::{Hangman, Running, Stopped};

#[test]
fn reveals_secret_word() {
    let secret_word = "abc";
    let game = Hangman::<Running>::start(secret_word, 123).unwrap();

    let stopped_game = Hangman::<Stopped>::from(game);

    assert!(stopped_game.to_string().contains(&format!(
        "Secret word was: {}\n",
        secret_word.to_ascii_uppercase()
    )));
}

#[test]
fn shows_remaining_failures() {
    let allowed_failures = 2;
    let game = Hangman::<Running>::start("abc", allowed_failures).unwrap();

    let stopped_game = Hangman::<Stopped>::from(game);

    assert!(stopped_game.to_string().contains(&format!(
        "You could have failed {allowed_failures} more guesses\n",
    )));
}

#[test]
fn shows_no_remaining_failures() {
    let allowed_failures = 1;
    let game = Hangman::<Running>::start("abc", allowed_failures).unwrap();
    let (_, game_state) = game.guess('x');

    let GameState::Lost(game) = game_state else {
        panic!("Expected game to be Lost");
    };

    assert!(!game.to_string().contains("You could have failed"));
}
