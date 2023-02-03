use std::collections::HashMap;

mod game_10_to_0;
mod game_25_to_0;

#[derive(Debug, PartialEq, Eq)]
pub enum GameResult {
    Win,
    Lose,
    Tie,
    Draw,
}

pub trait Move {}

pub trait PrimitiveValue {
    fn result(&self) -> Option<GameResult>;
}

pub trait Position {
    type GameMove: Move;
    type GamePrimitiveValue: PrimitiveValue;

    fn do_move(&self, mov: Self::GameMove) -> Self;
    fn generate_moves(&self) -> Vec<Self::GameMove>;
    fn primitive_value(&self) -> Self::GamePrimitiveValue;
}

struct Solver<T: Position> {
    memoized_results: HashMap<T, GameResult>,
}

impl<T: Position> Solver<T> {
    pub fn new(memoized_results: HashMap<T, GameResult>) -> Self {
        Self { memoized_results }
    }

    fn children(&self, position: T) -> Vec<T> {
        position
            .generate_moves()
            .into_iter()
            .map(|mov| position.do_move(mov))
            .collect()
    }

    pub fn solve(&self, position: T) -> GameResult {
        if let Some(result) = position.primitive_value().result() {
            return result;
        }

        let children_results = self
            .children(position)
            .into_iter()
            .map(|child| self.solve(child))
            .collect::<Vec<_>>();

        if children_results.iter().any(|r| *r == GameResult::Lose) {
            return GameResult::Win;
        }

        if children_results.iter().any(|r| *r == GameResult::Tie) {
            return GameResult::Tie;
        }

        GameResult::Lose
    }
}
