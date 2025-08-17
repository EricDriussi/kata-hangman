use crate::hangman::running_hangman::RunningHangman;
use crate::hangman::stopped_hangman::StoppedHangman;

pub enum GameState {
    InProgress(RunningHangman),
    Lost(StoppedHangman),
    Won(StoppedHangman),
}
