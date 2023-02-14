mod position_grp_elem;

pub use self::position_grp_elem::PositionGrpElem;

use super::{PlayerMove, PrimitiveValue};

pub trait Position<M, PV>: PositionGrpElem
where
    M: PlayerMove,
    PV: PrimitiveValue,
{
    fn do_move(&self, mov: M) -> Self;
    fn generate_moves(&self) -> Vec<M>;
    fn primitive_value(&self) -> PV;
}
