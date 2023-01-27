mod game;

trait Move {}

trait PrimitiveValue {}

trait Position {
    type T: Move;
    type V: PrimitiveValue;

    fn do_move(&mut self, mov: Self::T);
    fn generate_moves(&self) -> Vec<Self::T>;
    fn primitive_value(&self) -> Self::V;
}
