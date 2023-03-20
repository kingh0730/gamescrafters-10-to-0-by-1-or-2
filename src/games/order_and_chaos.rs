mod position;
mod recursive_value;

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

#[derive(Debug)]
enum OrderAndChaosPiece {
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
