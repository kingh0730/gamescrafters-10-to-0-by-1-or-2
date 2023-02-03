use crate::{GameResult, Move, Position, PrimitiveValue};

#[derive(Debug)]
enum TenToZeroPrimitiveValue {
    _Win,
    Lose,
    _Tie,
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
    fn result(&self) -> Option<GameResult> {
        match self {
            TenToZeroPrimitiveValue::_Win => Some(GameResult::Win),
            TenToZeroPrimitiveValue::Lose => Some(GameResult::Lose),
            TenToZeroPrimitiveValue::_Tie => Some(GameResult::Tie),
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
            _ => vec![],
        }
    }

    fn primitive_value(&self) -> TenToZeroPrimitiveValue {
        match self.remaining_count {
            0 => TenToZeroPrimitiveValue::Lose,
            _ => TenToZeroPrimitiveValue::NotPrimitive,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::TenToZeroPosition;
    use crate::Solver;
    use std::collections::HashMap;

    #[test]
    fn it_works() {
        let solver = Solver::new(HashMap::new());

        for i in (0..=10).rev() {
            let result = solver.solve(TenToZeroPosition { remaining_count: i });
            println!("{i}: {:?}", result);
        }
    }
}
