use std::hash::Hash;

pub trait Position<M, PV>: Eq + Hash {
    fn do_move(&self, mov: M) -> Self;
    fn generate_moves(&self) -> Vec<M>;
    fn primitive_value(&self) -> PV;
}
