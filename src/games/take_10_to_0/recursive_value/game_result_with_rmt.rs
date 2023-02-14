use crate::games::take_10_to_0::TenToZeroPrimitiveValue;
use crate::solver::{GameResult, GameResultWithRmt, RmtU32, ToRecursiveValue};

impl ToRecursiveValue<GameResultWithRmt> for TenToZeroPrimitiveValue {
    fn to_recursive_value(&self) -> Option<GameResultWithRmt> {
        match self {
            TenToZeroPrimitiveValue::_Win => Some(GameResultWithRmt {
                game_result: GameResult::Win,
                rmt: RmtU32::Val(0),
            }),
            TenToZeroPrimitiveValue::Lose => Some(GameResultWithRmt {
                game_result: GameResult::Lose,
                rmt: RmtU32::Val(0),
            }),
            TenToZeroPrimitiveValue::_Tie => Some(GameResultWithRmt {
                game_result: GameResult::Tie,
                rmt: RmtU32::Val(0),
            }),
            TenToZeroPrimitiveValue::NotPrimitive => None,
        }
    }
}
