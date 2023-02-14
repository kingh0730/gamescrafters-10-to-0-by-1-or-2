use crate::games::tic_tac_toe::TicTacToePrimitiveValue;
use crate::solver::{GameResult, ToRecursiveValue};

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
