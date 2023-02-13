mod recursive_value;

use std::collections::HashMap;
use std::hash::Hash;

pub use self::recursive_value::GameResult;

use self::recursive_value::RecursiveValue;

pub trait Move {}

pub trait PrimitiveValue {
    fn to_game_result(&self) -> Option<GameResult>;
    fn is_primitive(&self) -> bool;
}

pub trait Position: Eq + Hash {
    type GameMove: Move;
    type GamePrimitiveValue: PrimitiveValue;

    fn do_move(&self, mov: Self::GameMove) -> Self;
    fn generate_moves(&self) -> Vec<Self::GameMove>;
    fn primitive_value(&self) -> Self::GamePrimitiveValue;
}

#[derive(Debug)]
pub struct Solver<P: Position, V: RecursiveValue> {
    memoized_map: HashMap<P, V>,
}

impl<P: Position, V: RecursiveValue> Solver<P, V> {
    pub fn new(memoized_map: HashMap<P, V>) -> Self {
        Self { memoized_map }
    }

    fn children(&self, position: &P) -> Vec<P> {
        position
            .generate_moves()
            .into_iter()
            .map(|mov| position.do_move(mov))
            .collect()
    }
}

impl<P: Position> Solver<P, GameResult> {
    fn solve_not_memoized(&mut self, position: &P) -> GameResult {
        if let Some(result) = position.primitive_value().to_game_result() {
            return result;
        }

        let children_results = self
            .children(position)
            .into_iter()
            .map(|child| self.solve(child))
            .collect::<Vec<_>>();

        if children_results.iter().any(|&r| r == GameResult::Lose) {
            return GameResult::Win;
        }

        if children_results.iter().any(|&r| r == GameResult::Tie) {
            return GameResult::Tie;
        }

        GameResult::Lose
    }

    pub fn solve(&mut self, position: P) -> GameResult {
        if let Some(&result) = self.memoized_map.get(&position) {
            return result;
        }

        let result = self.solve_not_memoized(&position);

        self.memoized_map.insert(position, result);

        result
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::{GameResult, Position, PrimitiveValue, Solver};
    use crate::games::tic_tac_toe::{TicTacToePlayer, TicTacToePosition};

    #[test]
    fn it_works() {
        let mut solver = Solver::new(HashMap::new());

        let result = solver.solve(TicTacToePosition {
            board: [[None, None, None], [None, None, None], [None, None, None]],
            player: TicTacToePlayer::X,
        });

        assert_eq!(result, GameResult::Tie);
    }

    #[test]
    fn tic_tac_toe_counts() {
        let mut solver = Solver::new(HashMap::new());

        solver.solve(TicTacToePosition {
            board: [[None, None, None], [None, None, None], [None, None, None]],
            player: TicTacToePlayer::X,
        });

        let wins = solver
            .memoized_map
            .iter()
            .filter(|(_, &r)| r == GameResult::Win)
            .count();

        let loses = solver
            .memoized_map
            .iter()
            .filter(|(_, &r)| r == GameResult::Lose)
            .count();

        let ties = solver
            .memoized_map
            .iter()
            .filter(|(_, &r)| r == GameResult::Tie)
            .count();

        let total = solver.memoized_map.iter().count();

        let prim_wins = solver
            .memoized_map
            .iter()
            .filter(|(position, &r)| {
                r == GameResult::Win && position.primitive_value().is_primitive()
            })
            .count();

        let prim_loses = solver
            .memoized_map
            .iter()
            .filter(|(position, &r)| {
                r == GameResult::Lose && position.primitive_value().is_primitive()
            })
            .count();

        let prim_ties = solver
            .memoized_map
            .iter()
            .filter(|(position, &r)| {
                r == GameResult::Tie && position.primitive_value().is_primitive()
            })
            .count();

        let prim_total = solver
            .memoized_map
            .iter()
            .filter(|(position, _)| position.primitive_value().is_primitive())
            .count();

        assert_eq!(2836, wins);
        assert_eq!(1574, loses);
        assert_eq!(1068, ties);
        assert_eq!(5478, total);

        assert_eq!(0, prim_wins);
        assert_eq!(942, prim_loses);
        assert_eq!(16, prim_ties);
        assert_eq!(958, prim_total);

        println!("wins: {}", wins);
        println!("loses: {}", loses);
        println!("ties: {}", ties);
        println!("total: {}", total);

        println!("primitive wins: {}", prim_wins);
        println!("primitive loses: {}", prim_loses);
        println!("primitive ties: {}", prim_ties);
        println!("primitive total: {}", prim_total);
    }
}
