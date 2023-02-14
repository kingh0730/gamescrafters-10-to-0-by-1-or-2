use crate::games::take_10_to_0::TenToZeroPrimitiveValue;
use crate::solver::{GameResult, ToRecursiveValue};

impl ToRecursiveValue<GameResult> for TenToZeroPrimitiveValue {
    fn to_recursive_value(&self) -> Option<GameResult> {
        match self {
            TenToZeroPrimitiveValue::_Win => Some(GameResult::Win),
            TenToZeroPrimitiveValue::Lose => Some(GameResult::Lose),
            TenToZeroPrimitiveValue::_Tie => Some(GameResult::Tie),
            TenToZeroPrimitiveValue::NotPrimitive => None,
        }
    }
}
