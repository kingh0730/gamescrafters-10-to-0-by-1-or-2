use crate::games::tic_tac_toe_non_sq::TicTacToeNonSqPrimitiveValue;
use crate::solver::{GameResult, GameResultWithRmt, RmtU32, ToRecursiveValue};

impl ToRecursiveValue<GameResultWithRmt> for TicTacToeNonSqPrimitiveValue {
    fn to_recursive_value(&self) -> Option<GameResultWithRmt> {
        match self {
            TicTacToeNonSqPrimitiveValue::_Win => Some(GameResultWithRmt {
                game_result: GameResult::Win,
                rmt: RmtU32::Val(0),
            }),
            TicTacToeNonSqPrimitiveValue::Lose => Some(GameResultWithRmt {
                game_result: GameResult::Lose,
                rmt: RmtU32::Val(0),
            }),
            TicTacToeNonSqPrimitiveValue::Tie => Some(GameResultWithRmt {
                game_result: GameResult::Tie,
                rmt: RmtU32::Val(0),
            }),
            TicTacToeNonSqPrimitiveValue::NotPrimitive => None,
        }
    }
}
