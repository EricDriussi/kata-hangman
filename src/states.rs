#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum GameState {
    InProgress,
    Won,
    Lost,
}