use super::{Move, Position, PrimitiveValue};

enum GamePrimitiveValue {
    Win,
    Lose,
    Tie,
    NotPrimitive,
}
impl PrimitiveValue for GamePrimitiveValue {}

enum GameMove {
    Take1,
    Take2,
}
impl Move for GameMove {}

struct GamePosition {
    remaining_count: u32,
}

impl Position for GamePosition {
    type T = GameMove;
    type V = GamePrimitiveValue;

    fn do_move(&mut self, mov: GameMove) {
        match mov {
            GameMove::Take1 => self.remaining_count -= 1,
            GameMove::Take2 => self.remaining_count -= 2,
        }
    }

    fn generate_moves(&self) -> Vec<GameMove> {
        match self.remaining_count {
            2.. => vec![GameMove::Take1, GameMove::Take2],
            1 => vec![GameMove::Take1],
            _ => panic!("Invalid remaining count"),
        }
    }

    fn primitive_value(&self) -> GamePrimitiveValue {
        match self.remaining_count {
            0 => GamePrimitiveValue::Lose,
            _ => GamePrimitiveValue::NotPrimitive,
        }
    }
}
