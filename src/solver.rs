use std::collections::HashMap;
use std::hash::Hash;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
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

pub trait Position: Eq + Hash {
    type GameMove: Move;
    type GamePrimitiveValue: PrimitiveValue;

    fn do_move(&self, mov: Self::GameMove) -> Self;
    fn generate_moves(&self) -> Vec<Self::GameMove>;
    fn primitive_value(&self) -> Self::GamePrimitiveValue;
}

#[derive(Debug)]
pub struct Solver<T: Position> {
    pub memoized_results: HashMap<T, GameResult>,
}

impl<T: Position> Solver<T> {
    pub fn new(memoized_results: HashMap<T, GameResult>) -> Self {
        Self { memoized_results }
    }

    fn children(&self, position: &T) -> Vec<T> {
        position
            .generate_moves()
            .into_iter()
            .map(|mov| position.do_move(mov))
            .collect()
    }

    fn solve_not_memoized(&mut self, position: &T) -> GameResult {
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

    pub fn solve(&mut self, position: T) -> GameResult {
        if let Some(result) = self.memoized_results.get(&position) {
            return *result;
        }

        let result = self.solve_not_memoized(&position);

        self.memoized_results.insert(position, result);

        result
    }
}
