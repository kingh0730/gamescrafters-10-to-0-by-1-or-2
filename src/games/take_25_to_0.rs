use crate::solver::{GameResult, PlayerMove, Position, PrimitiveValue};

#[derive(Debug)]
enum TwentyFiveToZeroPrimitiveValue {
    _Win,
    Lose,
    _Tie,
    NotPrimitive,
}

#[derive(Debug)]
enum TwentyFiveToZeroMove {
    Take1,
    Take3,
    Take4,
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct TwentyFiveToZeroPosition {
    remaining_count: u32,
}

impl PlayerMove for TwentyFiveToZeroMove {}

impl PrimitiveValue for TwentyFiveToZeroPrimitiveValue {
    fn to_game_result(&self) -> Option<GameResult> {
        match self {
            TwentyFiveToZeroPrimitiveValue::_Win => Some(GameResult::Win),
            TwentyFiveToZeroPrimitiveValue::Lose => Some(GameResult::Lose),
            TwentyFiveToZeroPrimitiveValue::_Tie => Some(GameResult::Tie),
            TwentyFiveToZeroPrimitiveValue::NotPrimitive => None,
        }
    }

    fn is_primitive(&self) -> bool {
        match self {
            TwentyFiveToZeroPrimitiveValue::NotPrimitive => false,
            _ => true,
        }
    }
}

impl Position for TwentyFiveToZeroPosition {
    type GameMove = TwentyFiveToZeroMove;
    type GamePrimitiveValue = TwentyFiveToZeroPrimitiveValue;

    fn do_move(&self, mov: TwentyFiveToZeroMove) -> TwentyFiveToZeroPosition {
        let remaining_count = match mov {
            TwentyFiveToZeroMove::Take1 => self.remaining_count - 1,
            TwentyFiveToZeroMove::Take3 => self.remaining_count - 3,
            TwentyFiveToZeroMove::Take4 => self.remaining_count - 4,
        };

        TwentyFiveToZeroPosition { remaining_count }
    }

    fn generate_moves(&self) -> Vec<TwentyFiveToZeroMove> {
        match self.remaining_count {
            4.. => vec![
                TwentyFiveToZeroMove::Take1,
                TwentyFiveToZeroMove::Take3,
                TwentyFiveToZeroMove::Take4,
            ],
            3 => vec![TwentyFiveToZeroMove::Take1, TwentyFiveToZeroMove::Take3],
            1..=2 => vec![TwentyFiveToZeroMove::Take1],
            _ => vec![],
        }
    }

    fn primitive_value(&self) -> TwentyFiveToZeroPrimitiveValue {
        match self.remaining_count {
            0 => TwentyFiveToZeroPrimitiveValue::Lose,
            _ => TwentyFiveToZeroPrimitiveValue::NotPrimitive,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::TwentyFiveToZeroPosition;
    use crate::solver::{GameResult, Solver};

    #[test]
    fn it_works() {
        let mut solver = Solver::new(HashMap::new());

        for i in (0..=25).rev() {
            let result = solver.solve(TwentyFiveToZeroPosition { remaining_count: i });

            match i % 7 {
                0 => assert_eq!(result, GameResult::Lose),
                1 => assert_eq!(result, GameResult::Win),
                2 => assert_eq!(result, GameResult::Lose),
                3 => assert_eq!(result, GameResult::Win),
                4 => assert_eq!(result, GameResult::Win),
                5 => assert_eq!(result, GameResult::Win),
                6 => assert_eq!(result, GameResult::Win),
                _ => (),
            };

            println!("{i}: {:?}", result);
        }
    }
}
