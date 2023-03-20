use crate::games::order_and_chaos::OrderAndChaosPrimitiveValue;
use crate::solver::{GameResult, GameResultWithRmt, RmtU32, ToRecursiveValue};

impl ToRecursiveValue<GameResultWithRmt> for OrderAndChaosPrimitiveValue {
    fn to_recursive_value(&self) -> Option<GameResultWithRmt> {
        match self {
            OrderAndChaosPrimitiveValue::Win => Some(GameResultWithRmt {
                game_result: GameResult::Win,
                rmt: RmtU32::Val(0),
            }),
            OrderAndChaosPrimitiveValue::Lose => Some(GameResultWithRmt {
                game_result: GameResult::Lose,
                rmt: RmtU32::Val(0),
            }),
            OrderAndChaosPrimitiveValue::Tie => Some(GameResultWithRmt {
                game_result: GameResult::Tie,
                rmt: RmtU32::Val(0),
            }),
            OrderAndChaosPrimitiveValue::NotPrimitive => None,
        }
    }
}
