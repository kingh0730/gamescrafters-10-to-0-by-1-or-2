mod game;

trait Move {}

trait PrimitiveValue {}

trait Position {
    type GameMove: Move;
    type GamePrimitiveValue: PrimitiveValue;

    fn do_move(&mut self, mov: Self::GameMove);
    fn generate_moves(&self) -> Vec<Self::GameMove>;
    fn primitive_value(&self) -> Self::GamePrimitiveValue;
}
