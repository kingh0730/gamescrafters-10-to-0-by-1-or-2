trait Move {}

trait PrimitiveValue {}

trait Position<T: Move, V: PrimitiveValue> {
    fn do_move(&mut self, mov: T);
    fn generate_moves(&self) -> Vec<T>;
    fn primitive_value(&self) -> V;
}
