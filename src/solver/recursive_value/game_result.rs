use super::RecursiveValue;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum GameResult {
    Win,
    Lose,
    Tie,
    Draw,
}

impl RecursiveValue for GameResult {
    fn recursion_step(children: &[Self]) -> Self {
        if children.iter().any(|&r| r == GameResult::Lose) {
            return GameResult::Win;
        }

        if children.iter().any(|&r| r == GameResult::Tie) {
            return GameResult::Tie;
        }

        if children.iter().any(|&r| r == GameResult::Draw) {
            return GameResult::Draw;
        }

        GameResult::Lose
    }
}