use hangman::hangman::{GuessResult, Hangman};
use hangman::state::GameState;
use std::io;

fn main() {
    let mut game = start_game();

    while game.state() == GameState::InProgress {
        println!("Current state: {:?}", game.state());
        println!("Secret word: {}", game.display_word());
        println!("Correctly guessed characters: {:?}", game.already_guessed());
        println!("Allowed failures left: {}", game.remaining_failures());

        println!("Enter your guess (single character):");
        let guess = read_stdin().chars().next().unwrap_or_default();

        match game.guess(guess) {
            GuessResult::Correct => println!("Good guess!"),
            GuessResult::Incorrect => println!("Wrong guess!"),
            GuessResult::Duplicate => println!("You already guessed that character!"),
            GuessResult::InvalidCharacter => println!("Invalid character! Please enter a letter."),
            GuessResult::GameNotInProgress => {
                println!("The game is not in progress. Exiting...");
                break;
            }
        }
    }
}

fn start_game() -> Hangman {
    println!("Enter the secret word:");
    let secret_word = read_stdin();

    println!("Enter the allowed number of failures:");
    let allowed_failures = read_int();

    Hangman::init(&secret_word, allowed_failures).unwrap_or_else(|_| start_game())
}

fn read_stdin() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("I/O ERROR!!");
    input.trim().to_string()
}

fn read_int() -> usize {
    let input = read_stdin();
    input.trim().parse::<usize>().unwrap_or_else(|_| {
        eprintln!("That's doesn't make sense...");
        read_int()
    })
}
