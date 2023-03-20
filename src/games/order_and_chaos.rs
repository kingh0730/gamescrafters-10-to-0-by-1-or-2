mod position;
mod recursive_value;

pub use self::position::OrderAndChaosPlayer;
pub use self::position::OrderAndChaosPosition;

use crate::solver::{PlayerMove, PrimitiveValue};

const WIDTH: usize = 2;
const HEIGHT: usize = 2;
const K_IN_A_ROW: usize = 2;

// Check bounds
const _: () = assert!(WIDTH <= 10);
const _: () = assert!(HEIGHT <= 10);
const _: () = assert!(K_IN_A_ROW <= 10 && K_IN_A_ROW >= 1);

// Start position
impl OrderAndChaosPosition {
    pub fn start() -> OrderAndChaosPosition {
        OrderAndChaosPosition {
            board: [[None, None], [None, None]],
            player: OrderAndChaosPlayer::Order,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum OrderAndChaosPrimitiveValue {
    Win,
    Lose,
    Tie,
    NotPrimitive,
}

impl PrimitiveValue for OrderAndChaosPrimitiveValue {
    fn is_primitive(&self) -> bool {
        match self {
            OrderAndChaosPrimitiveValue::NotPrimitive => false,
            _ => true,
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub enum OrderAndChaosPiece {
    X,
    O,
}

#[derive(Debug)]
pub struct OrderAndChaosMove {
    piece: OrderAndChaosPiece,
    x: usize,
    y: usize,
}

impl PlayerMove for OrderAndChaosMove {}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::OrderAndChaosPosition;
    use crate::solver::{GameResult, Solver};

    #[test]
    fn test_order_and_chaos() {
        let mut solver = Solver::<_, _, _, GameResult>::new(HashMap::new());

        let result = solver.solve(OrderAndChaosPosition::start());

        println!("{:?}", result);
    }
}
