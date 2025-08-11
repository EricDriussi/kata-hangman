use hangman::hangman::{GameState, Hangman, Running};
use hangman::results::GuessResult;
use std::io;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    clear_screen();
    let game = start_game();
    run_round(game);
}

fn run_round(game: Hangman<Running>) {
    clear_screen();
    println!("{game}");
    println!("\nTake a guess!");

    let guess = read_char();
    let (guess_result, state) = game.guess(guess);

    match guess_result {
        GuessResult::Correct => println!("Correct!"),
        GuessResult::Incorrect => println!("Not quite..."),
        GuessResult::Duplicate => println!("You already tried that!"),
        GuessResult::Invalid => println!("WTF was that?! Just enter a character!"),
    }

    sleep(Duration::from_secs(1));

    match state {
        GameState::InProgress(running_game) => {
            run_round(running_game);
        }
        GameState::Won(stopped_game) => {
            clear_screen();
            println!("You won!!");
            println!("{stopped_game}");
        }
        GameState::Lost(stopped_game) => {
            clear_screen();
            println!("You lost :(");
            println!("{stopped_game}");
        }
    }
}

fn start_game() -> Hangman<Running> {
    println!("Enter the secret word:");
    let secret_word = read_stdin();

    println!("Enter the allowed number of failures:");
    let allowed_failures = read_int();

    Hangman::<Running>::start(&secret_word, allowed_failures).unwrap_or_else(|e| {
        clear_screen();
        println!("{e}");
        println!("Try again...");
        sleep(Duration::from_secs(1));
        start_game()
    })
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
    clear_screen();
    let input = read_stdin();
    input.trim().parse::<char>().unwrap_or_else(|_| {
        println!("Only ONE character! Try again!");
        read_char()
    })
}

fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
}
