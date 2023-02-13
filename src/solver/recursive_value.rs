pub trait RecursiveValue {
    fn recursion_step(children: &[Self]) -> Self
    where
        Self: Sized;
}

pub trait ToRecursiveValue<RV: RecursiveValue> {
    fn to_recursive_value(&self) -> Option<RV>;
}

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

        GameResult::Lose
    }
}
