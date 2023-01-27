use crate::{GameResult, Move, Position, PrimitiveValue};

#[derive(Debug)]
enum TwentyFiveToZeroPrimitiveValue {
    Win,
    Lose,
    Tie,
    NotPrimitive,
}

#[derive(Debug)]
enum TwentyFiveToZeroMove {
    Take1,
    Take2,
}

#[derive(Debug)]
struct TwentyFiveToZeroPosition {
    remaining_count: u32,
}

impl Move for TwentyFiveToZeroMove {}

impl PrimitiveValue for TwentyFiveToZeroPrimitiveValue {
    fn result(&self) -> Option<GameResult> {
        match self {
            TwentyFiveToZeroPrimitiveValue::Win => Some(GameResult::Win),
            TwentyFiveToZeroPrimitiveValue::Lose => Some(GameResult::Lose),
            TwentyFiveToZeroPrimitiveValue::Tie => Some(GameResult::Tie),
            TwentyFiveToZeroPrimitiveValue::NotPrimitive => None,
        }
    }
}

impl Position for TwentyFiveToZeroPosition {
    type GameMove = TwentyFiveToZeroMove;
    type GamePrimitiveValue = TwentyFiveToZeroPrimitiveValue;

    fn do_move(&self, mov: TwentyFiveToZeroMove) -> TwentyFiveToZeroPosition {
        let remaining_count = match mov {
            TwentyFiveToZeroMove::Take1 => self.remaining_count - 1,
            TwentyFiveToZeroMove::Take2 => self.remaining_count - 2,
        };

        TwentyFiveToZeroPosition { remaining_count }
    }

    fn generate_moves(&self) -> Vec<TwentyFiveToZeroMove> {
        match self.remaining_count {
            2.. => vec![TwentyFiveToZeroMove::Take1, TwentyFiveToZeroMove::Take2],
            1 => vec![TwentyFiveToZeroMove::Take1],
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
    use super::TwentyFiveToZeroPosition;
    use crate::solve;

    #[test]
    fn it_works() {
        for i in (0..=10).rev() {
            let result = solve(TwentyFiveToZeroPosition { remaining_count: i });
            println!("{i}: {:?}", result);
        }
    }
}
