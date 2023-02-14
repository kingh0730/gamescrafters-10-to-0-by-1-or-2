use crate::games::tic_tac_toe::TicTacToePrimitiveValue;
use crate::solver::{GameResult, GameResultWithRmt, RmtU32, ToRecursiveValue};

impl ToRecursiveValue<GameResultWithRmt> for TicTacToePrimitiveValue {
    fn to_recursive_value(&self) -> Option<GameResultWithRmt> {
        match self {
            TicTacToePrimitiveValue::_Win => Some(GameResultWithRmt {
                game_result: GameResult::Win,
                rmt: RmtU32::Val(0),
            }),
            TicTacToePrimitiveValue::Lose => Some(GameResultWithRmt {
                game_result: GameResult::Lose,
                rmt: RmtU32::Val(0),
            }),
            TicTacToePrimitiveValue::Tie => Some(GameResultWithRmt {
                game_result: GameResult::Tie,
                rmt: RmtU32::Val(0),
            }),
            TicTacToePrimitiveValue::NotPrimitive => None,
        }
    }
}
