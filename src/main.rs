use hangman::errors::GuessError;
use hangman::hangman::Hangman;
use hangman::results::GuessResult;
use hangman::states::GameState;
use std::io;
use std::thread::sleep;
use std::time::Duration;

// TODO: review the tests!
fn main() {
    clear_screen();
    let (mut game, secret_word) = start_game();

    while game.state() == GameState::InProgress {
        clear_screen();

        println!("Current state: {:?}", game.state());
        println!("Secret word: {}", game.display_word());
        println!("Allowed failures left: {}", game.remaining_failures());
        println!("Already guessed characters: {}", game.already_guessed());

        println!();
        println!("Take a guess!");
        let guess = read_char();

        clear_screen();
        match game.guess(guess) {
            Ok(guess_result) => match guess_result {
                GuessResult::Correct => println!("Correct!"),
                GuessResult::Incorrect => println!("Not quite..."),
                GuessResult::Duplicate => println!("You already tried that!"),
            },
            Err(guess_error) => match guess_error {
                GuessError::InvalidCharacter => println!("WTF was that?! Just enter a character!"),
                GuessError::GameNotInProgress => {
                    println!("This should never happen :(");
                    break;
                }
            },
        }
        sleep(Duration::from_secs(1));
    }

    if game.state() == GameState::Won {
        println!("Yay! You guessed '{secret_word}'!");
    } else {
        println!("Game over! The secret word was: {secret_word}");
    }
}

fn start_game() -> (Hangman, String) {
    println!("Enter the secret word:");
    let secret_word = read_stdin();

    println!("Enter the allowed number of failures:");
    let allowed_failures = read_int();

    match Hangman::start(&secret_word, allowed_failures) {
        Ok(game) => (game, secret_word),
        Err(e) => {
            clear_screen();
            println!("{e}");
            println!("Try again...");
            sleep(Duration::from_secs(1));
            start_game()
        }
    }
}

fn read_stdin() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("I/O ERROR!!");
    input.trim().to_string()
}

fn read_int() -> isize {
    let input = read_stdin();
    input.trim().parse::<isize>().unwrap_or_else(|_| {
        println!("That's doesn't make sense... Try again!");
        read_int()
    })
}

fn read_char() -> char {
    let input = read_stdin();
    input.trim().parse::<char>().unwrap_or_else(|_| {
        println!("Only ONE character! Try again!");
        read_char()
    })
}

fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
}
