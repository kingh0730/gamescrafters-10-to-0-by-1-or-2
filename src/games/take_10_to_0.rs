mod recursive_value;

use crate::solver::{PlayerMove, Position, PositionKey, PrimitiveValue};

#[derive(Debug)]
pub enum TenToZeroPrimitiveValue {
    _Win,
    Lose,
    _Tie,
    NotPrimitive,
}

#[derive(Debug)]
pub enum TenToZeroMove {
    Take1,
    Take2,
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct TenToZeroPosition {
    pub remaining_count: u32,
}

impl PlayerMove for TenToZeroMove {}

impl PrimitiveValue for TenToZeroPrimitiveValue {
    fn is_primitive(&self) -> bool {
        match self {
            TenToZeroPrimitiveValue::NotPrimitive => false,
            _ => true,
        }
    }
}

impl PositionKey for TenToZeroPosition {}

impl Position<TenToZeroMove, TenToZeroPrimitiveValue> for TenToZeroPosition {
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
    use std::collections::HashMap;

    use super::TenToZeroPosition;
    use crate::solver::{GameResult, Solver};

    #[test]
    fn it_works() {
        let mut solver = Solver::<_, _, _, GameResult>::new(HashMap::new());

        for i in (0..=10).rev() {
            let result = solver.solve(TenToZeroPosition { remaining_count: i });

            match i % 3 {
                0 => assert_eq!(result, GameResult::Lose),
                1 => assert_eq!(result, GameResult::Win),
                2 => assert_eq!(result, GameResult::Win),
                _ => (),
            };

            println!("{i}: {:?}", result);
        }
    }

    #[test]
    fn test_memoization() {
        let mut solver = Solver::<_, _, _, GameResult>::new(HashMap::new());

        for i in (0..=100).rev() {
            let result = solver.solve(TenToZeroPosition { remaining_count: i });

            match i % 3 {
                0 => assert_eq!(result, GameResult::Lose),
                1 => assert_eq!(result, GameResult::Win),
                2 => assert_eq!(result, GameResult::Win),
                _ => (),
            };

            println!("{i}: {:?}", result);
        }

        println!("{:#?}", solver);
    }
}
