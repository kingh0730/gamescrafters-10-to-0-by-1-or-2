enum PrimitiveValue {
    Win,
    Lose,
    Tie,
    NotPrimitive,
}

enum Move {
    Take1,
    Take2,
}

struct Position {
    remaining_count: u32,
}

impl Position {
    fn do_move(&mut self, mov: Move) {
        match mov {
            Move::Take1 => self.remaining_count -= 1,
            Move::Take2 => self.remaining_count -= 2,
        }
    }

    fn generate_moves(&self) -> Vec<Move> {
        match self.remaining_count {
            2.. => vec![Move::Take1, Move::Take2],
            1 => vec![Move::Take1],
            _ => panic!("Invalid remaining count"),
        }
    }

    fn primitive_value(&self) -> PrimitiveValue {
        match self.remaining_count {
            0 => PrimitiveValue::Lose,
            _ => PrimitiveValue::NotPrimitive,
        }
    }
}
