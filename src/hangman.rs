use crate::errors::InitError;
// use crate::result::GuessResult;
use crate::state::GameState;
use std::collections::{HashMap, HashSet};

#[derive(Debug, PartialEq, Eq)]
pub struct Hangman {
    secret_word: String,
    incorrect_guess_limit: usize,
    incorrect_guesses: HashSet<char>,
    guessed_letters: HashSet<char>,
    revealed_word_state: HashMap<char, bool>,
    state: GameState,
}

impl Hangman {
    pub fn init(secret_word: &str, incorrect_guess_limit: usize) -> Result<Self, InitError> {
        if incorrect_guess_limit < 1 {
            return Err(InitError::NotEnoughGuesses);
        }

        if secret_word.is_empty() {
            return Err(InitError::EmptySecretWord);
        }

        if secret_word.chars().any(|ch| !ch.is_ascii_alphabetic()) {
            return Err(InitError::NonAlphabeticCharacters);
        }

        Ok(Hangman {
            secret_word: secret_word.to_string(),
            incorrect_guess_limit,
            incorrect_guesses: HashSet::new(),
            guessed_letters: HashSet::new(),
            revealed_word_state: secret_word.chars().map(|ch| (ch, false)).collect(),
            state: GameState::InProgress,
        })
    }

    pub fn state(&self) -> GameState {
        self.state
    }

    // pub fn incorrect_guesses_count(&self) -> usize {
    //     self.incorrect_guesses.len()
    // }
    //
    // pub fn get_incorrect_guesses(&self) -> &HashSet<char> {
    //     &self.incorrect_guesses
    // }
    //
    // pub fn display_word(&self) -> String {
    //     self.secret_word
    //         .chars()
    //         .map(|c| {
    //             if *self.revealed_word_state.get(&c).unwrap_or(&false) {
    //                 c
    //             } else {
    //                 '_'
    //             }
    //         })
    //         .collect()
    // }
    //
    // pub fn guess(&mut self, letter: char) -> GuessResult {
    //     if !self.is_in_progress() {
    //         return GuessResult::Duplicate; // Or a specific "GameEnded" result, depending on desired behavior
    //     }
    //
    //     let upper_letter = letter.to_ascii_uppercase();
    //
    //     if !upper_letter.is_ascii_alphabetic() {
    //         return GuessResult::InvalidCharacter;
    //     }
    //
    //     if self.guessed_letters.contains(&upper_letter) {
    //         return GuessResult::Duplicate;
    //     }
    //
    //     self.guessed_letters.insert(upper_letter);
    //
    //     if self.secret_word.contains(upper_letter) {
    //         self.revealed_word_state.insert(upper_letter, true);
    //         self.check_game_state();
    //         GuessResult::Correct
    //     } else {
    //         self.incorrect_guesses.insert(upper_letter);
    //         self.check_game_state();
    //         GuessResult::Incorrect
    //     }
    // }
    //
    // fn check_game_state(&mut self) {
    //     if self.is_word_revealed() {
    //         self.game_state = GameState::Won;
    //     } else if self.incorrect_guesses.len() >= self.incorrect_guess_limit {
    //         self.game_state = GameState::Lost;
    //     } else {
    //         self.game_state = GameState::InProgress;
    //     }
    // }
    //
    // fn is_word_revealed(&self) -> bool {
    //     self.revealed_word_state.values().all(|&revealed| revealed)
    // }
}
