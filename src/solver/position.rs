use std::hash::Hash;

use super::{PlayerMove, PrimitiveValue};

pub trait Position<M, PV>: Eq + Hash
where
    M: PlayerMove,
    PV: PrimitiveValue,
{
    fn do_move(&self, mov: M) -> Self;
    fn generate_moves(&self) -> Vec<M>;
    fn primitive_value(&self) -> PV;
}
