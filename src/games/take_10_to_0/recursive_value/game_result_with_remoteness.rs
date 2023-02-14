use crate::games::take_10_to_0::TenToZeroPrimitiveValue;
use crate::solver::{GameResult, GameResultWithRemoteness, RemotenessU32, ToRecursiveValue};

impl ToRecursiveValue<GameResultWithRemoteness> for TenToZeroPrimitiveValue {
    fn to_recursive_value(&self) -> Option<GameResultWithRemoteness> {
        match self {
            TenToZeroPrimitiveValue::_Win => Some(GameResultWithRemoteness {
                game_result: GameResult::Win,
                remoteness: RemotenessU32::Val(0),
            }),
            TenToZeroPrimitiveValue::Lose => Some(GameResultWithRemoteness {
                game_result: GameResult::Lose,
                remoteness: RemotenessU32::Val(0),
            }),
            TenToZeroPrimitiveValue::_Tie => Some(GameResultWithRemoteness {
                game_result: GameResult::Tie,
                remoteness: RemotenessU32::Val(0),
            }),
            TenToZeroPrimitiveValue::NotPrimitive => None,
        }
    }
}
