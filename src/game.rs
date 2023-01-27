use super::{Move, Position, PrimitiveValue, Result};

#[derive(Debug)]
enum TenToZeroPrimitiveValue {
    Win,
    Lose,
    Tie,
    NotPrimitive,
}

#[derive(Debug)]
enum TenToZeroMove {
    Take1,
    Take2,
}

#[derive(Debug)]
struct TenToZeroPosition {
    remaining_count: u32,
}

impl Move for TenToZeroMove {}

impl PrimitiveValue for TenToZeroPrimitiveValue {
    fn result(&self) -> Option<Result> {
        match self {
            TenToZeroPrimitiveValue::Win => Some(Result::Win),
            TenToZeroPrimitiveValue::Lose => Some(Result::Lose),
            TenToZeroPrimitiveValue::Tie => Some(Result::Tie),
            TenToZeroPrimitiveValue::NotPrimitive => None,
        }
    }
}

impl Position for TenToZeroPosition {
    type GameMove = TenToZeroMove;
    type GamePrimitiveValue = TenToZeroPrimitiveValue;

    fn do_move(&self, mov: TenToZeroMove) -> TenToZeroPosition {
        let remaining_count = match mov {
            TenToZeroMove::Take1 => self.remaining_count - 1,
            TenToZeroMove::Take2 => self.remaining_count - 2,
        };

        TenToZeroPosition { remaining_count }
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
