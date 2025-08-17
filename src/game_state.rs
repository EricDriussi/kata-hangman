use crate::hangman::running::RunningHangman;
use crate::hangman::stopped::StoppedHangman;

pub enum GameState {
    InProgress(RunningHangman),
    Lost(StoppedHangman),
    Won(StoppedHangman),
}
