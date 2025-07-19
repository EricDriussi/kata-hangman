use crate::errors::InitError;
// use crate::results::GuessResult;
use crate::state::GameState;
use std::collections::{HashMap, HashSet};

#[derive(Debug, PartialEq, Eq)]
pub struct Hangman {
    word: String,
    max_failed_guesses: isize,
    failed_guesses: usize,
    guessed_letters: HashSet<char>,
    word_progress: HashMap<char, bool>,
    state: GameState,
}

impl Hangman {
    pub fn init(word: &str, max_failed_guesses: isize) -> Result<Self, InitError> {
        if max_failed_guesses < 1 {
            return Err(InitError::NotEnoughGuesses);
        }

        if word.is_empty() {
            return Err(InitError::EmptySecretWord);
        }

        if word.chars().any(|ch| !ch.is_alphabetic()) {
            return Err(InitError::NonAlphabeticCharacters);
        }

        Ok(Hangman {
            word: word.to_uppercase().to_string(),
            max_failed_guesses,
            failed_guesses: 0,
            guessed_letters: HashSet::new(),
            word_progress: word.to_uppercase().chars().map(|ch| (ch, false)).collect(),
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

    // pub fn guess(&mut self, letter: char) -> GuessResult {
    //     if self.state != GameState::InProgress {
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
    //     if self.word.contains(upper_letter) {
    //         self.word_progress.insert(upper_letter, true);
    //         self.check_game_state();
    //         GuessResult::Correct
    //     } else {
    //         self.guessed_letters.insert(upper_letter);
    //         self.check_game_state();
    //         GuessResult::Incorrect
    //     }
    // }
    //
    // fn check_game_state(&mut self) {
    //     if self.is_word_revealed() {
    //         self.state = GameState::Won;
    //     } else if self.failed_guesses >= self.max_failed_guesses {
    //         self.state = GameState::Lost;
    //     } else {
    //         self.state = GameState::InProgress;
    //     }
    // }
    //
    // fn is_word_revealed(&self) -> bool {
    //     self.word_progress.values().all(|&revealed| revealed)
    // }
}
