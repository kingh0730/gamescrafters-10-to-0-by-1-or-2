use super::{Move, Position, PrimitiveValue};

enum TenToZeroPrimitiveValue {
    Win,
    Lose,
    Tie,
    NotPrimitive,
}
impl PrimitiveValue for TenToZeroPrimitiveValue {}

enum TenToZeroMove {
    Take1,
    Take2,
}
impl Move for TenToZeroMove {}

struct TenToZeroPosition {
    remaining_count: u32,
}

impl Position for TenToZeroPosition {
    type GameMove = TenToZeroMove;
    type GamePrimitiveValue = TenToZeroPrimitiveValue;

    fn do_move(&mut self, mov: TenToZeroMove) {
        match mov {
            TenToZeroMove::Take1 => self.remaining_count -= 1,
            TenToZeroMove::Take2 => self.remaining_count -= 2,
        }
    }

    fn generate_moves(&self) -> Vec<TenToZeroMove> {
        match self.remaining_count {
            2.. => vec![TenToZeroMove::Take1, TenToZeroMove::Take2],
            1 => vec![TenToZeroMove::Take1],
            _ => panic!("Invalid remaining count"),
        }
    }

    fn primitive_value(&self) -> TenToZeroPrimitiveValue {
        match self.remaining_count {
            0 => TenToZeroPrimitiveValue::Lose,
            _ => TenToZeroPrimitiveValue::NotPrimitive,
        }
    }
}
