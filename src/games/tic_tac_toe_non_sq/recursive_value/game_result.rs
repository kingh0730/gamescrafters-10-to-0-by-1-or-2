use crate::games::tic_tac_toe_non_sq::TicTacToeNonSqPrimitiveValue;
use crate::solver::{GameResult, ToRecursiveValue};

impl ToRecursiveValue<GameResult> for TicTacToeNonSqPrimitiveValue {
    fn to_recursive_value(&self) -> Option<GameResult> {
        match self {
            TicTacToeNonSqPrimitiveValue::_Win => Some(GameResult::Win),
            TicTacToeNonSqPrimitiveValue::Lose => Some(GameResult::Lose),
            TicTacToeNonSqPrimitiveValue::Tie => Some(GameResult::Tie),
            TicTacToeNonSqPrimitiveValue::NotPrimitive => None,
        }
    }
}
