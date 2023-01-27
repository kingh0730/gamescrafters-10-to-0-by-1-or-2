mod game;

#[derive(Debug, PartialEq, Eq)]
enum Result {
    Win,
    Lose,
    Tie,
    Draw,
}

trait Move {}

trait PrimitiveValue {
    fn result(&self) -> Option<Result>;
}

trait Position {
    type GameMove: Move;
    type GamePrimitiveValue: PrimitiveValue;

    fn do_move(&self, mov: Self::GameMove) -> Self;
    fn generate_moves(&self) -> Vec<Self::GameMove>;
    fn primitive_value(&self) -> Self::GamePrimitiveValue;
}

fn children<T: Position>(position: T) -> Vec<T> {
    position
        .generate_moves()
        .into_iter()
        .map(|mov| position.do_move(mov))
        .collect()
}

fn solve<T: Position>(position: T) -> Result {
    if let Some(result) = position.primitive_value().result() {
        return result;
    }

    let mut children_results = children(position).into_iter().map(|child| solve(child));

    if children_results.any(|result| result == Result::Lose) {
        return Result::Win;
    }

    if children_results.any(|result| result == Result::Tie) {
        return Result::Tie;
    }

    Result::Lose
}
