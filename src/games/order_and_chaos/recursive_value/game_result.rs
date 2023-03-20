use crate::games::order_and_chaos::OrderAndChaosPrimitiveValue;
use crate::solver::{GameResult, ToRecursiveValue};

impl ToRecursiveValue<GameResult> for OrderAndChaosPrimitiveValue {
    fn to_recursive_value(&self) -> Option<GameResult> {
        match self {
            OrderAndChaosPrimitiveValue::Win => Some(GameResult::Win),
            OrderAndChaosPrimitiveValue::Lose => Some(GameResult::Lose),
            OrderAndChaosPrimitiveValue::Tie => Some(GameResult::Tie),
            OrderAndChaosPrimitiveValue::NotPrimitive => None,
        }
    }
}
