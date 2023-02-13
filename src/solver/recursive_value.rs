pub trait RecursiveValue {}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum GameResult {
    Win,
    Lose,
    Tie,
    Draw,
}

impl RecursiveValue for GameResult {}
