mod position;

pub use self::position::TicTacToePosition;

use crate::solver::{GameResult, PlayerMove, PrimitiveValue, ToRecursiveValue};

const LENGTH: usize = 3;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum TicTacToePlayer {
    X,
    O,
}

impl TicTacToePlayer {
    fn next_player(&self) -> TicTacToePlayer {
        match self {
            TicTacToePlayer::X => TicTacToePlayer::O,
            TicTacToePlayer::O => TicTacToePlayer::X,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum TicTacToePrimitiveValue {
    _Win,
    Lose,
    Tie,
    NotPrimitive,
}

#[derive(Debug)]
pub struct TicTacToeMove {
    x: usize,
    y: usize,
}

impl PlayerMove for TicTacToeMove {}

impl PrimitiveValue for TicTacToePrimitiveValue {
    fn is_primitive(&self) -> bool {
        match self {
            TicTacToePrimitiveValue::NotPrimitive => false,
            _ => true,
        }
    }
}

impl ToRecursiveValue<GameResult> for TicTacToePrimitiveValue {
    fn to_recursive_value(&self) -> Option<GameResult> {
        match self {
            TicTacToePrimitiveValue::_Win => Some(GameResult::Win),
            TicTacToePrimitiveValue::Lose => Some(GameResult::Lose),
            TicTacToePrimitiveValue::Tie => Some(GameResult::Tie),
            TicTacToePrimitiveValue::NotPrimitive => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::{TicTacToePlayer, TicTacToePosition};
    use crate::solver::{GameResult, Solver};

    #[test]
    fn it_works() {
        let mut solver = Solver::new(HashMap::new());

        let result = solver.solve(TicTacToePosition {
            board: [[None, None, None], [None, None, None], [None, None, None]],
            player: TicTacToePlayer::X,
        });

        assert_eq!(result, GameResult::Tie);
    }
}
