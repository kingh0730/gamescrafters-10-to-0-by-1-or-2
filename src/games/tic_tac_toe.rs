mod position;
mod recursive_value;

pub use self::position::TicTacToePlayer;
pub use self::position::TicTacToePosition;
pub use self::position::TicTacToePositionGrpElem;

use crate::solver::{PlayerMove, PrimitiveValue};

const LENGTH: usize = 3;

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

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::{TicTacToePlayer, TicTacToePosition};
    use crate::solver::{GameResult, Solver};

    #[test]
    fn it_works() {
        let mut solver = Solver::new(HashMap::new());

        let result: GameResult = solver.solve(TicTacToePosition {
            board: [[None, None, None], [None, None, None], [None, None, None]],
            player: TicTacToePlayer::X,
        });

        assert_eq!(result, GameResult::Tie);
    }
}
