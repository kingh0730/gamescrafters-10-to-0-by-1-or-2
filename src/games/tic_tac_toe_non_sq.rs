mod position;
mod recursive_value;

pub use self::position::TicTacToeNonSqPlayer;
pub use self::position::TicTacToeNonSqPosition;

use crate::solver::{PlayerMove, PrimitiveValue};

// ! used "as i32" sometimes
const WIDTH: usize = 3;
const HEIGHT: usize = 1;
const K_IN_A_ROW: usize = 3;

#[derive(Debug, PartialEq, Eq)]
pub enum TicTacToeNonSqPrimitiveValue {
    _Win,
    Lose,
    Tie,
    NotPrimitive,
}

impl PrimitiveValue for TicTacToeNonSqPrimitiveValue {
    fn is_primitive(&self) -> bool {
        match self {
            TicTacToeNonSqPrimitiveValue::NotPrimitive => false,
            _ => true,
        }
    }
}

#[derive(Debug)]
pub struct TicTacToeNonSqMove {
    x: usize,
    y: usize,
}

impl PlayerMove for TicTacToeNonSqMove {}
