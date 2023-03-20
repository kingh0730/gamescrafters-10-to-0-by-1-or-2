mod position;
mod recursive_value;

pub use self::position::OrderAndChaosPlayer;
pub use self::position::OrderAndChaosPosition;

use crate::solver::{PlayerMove, PrimitiveValue};

// TODO Check bounds
const WIDTH: usize = 3;
const HEIGHT: usize = 3;
const K_IN_A_ROW: usize = 3;

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

    use super::{OrderAndChaosPlayer, OrderAndChaosPosition};
    use crate::solver::{GameResult, Solver};

    #[test]
    fn test_order_and_chaos() {
        let mut solver = Solver::<_, _, _, GameResult>::new(HashMap::new());

        let result = solver.solve(OrderAndChaosPosition {
            board: [[None, None, None], [None, None, None], [None, None, None]],
            player: OrderAndChaosPlayer::Order,
        });

        println!("{:?}", result);
    }
}
