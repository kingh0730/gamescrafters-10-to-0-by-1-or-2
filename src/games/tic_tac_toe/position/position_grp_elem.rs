use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};

use crate::games::tic_tac_toe::{TicTacToeMove, TicTacToePrimitiveValue};
use crate::solver::{Position, PositionKey};

use super::TicTacToePosition;

pub struct TicTacToePositionD4Eq {
    pub position: TicTacToePosition,
}

impl PositionKey for TicTacToePositionD4Eq {}

impl PartialEq for TicTacToePositionD4Eq {
    fn eq(&self, other: &Self) -> bool {
        (self.position == other.position)
            || (self.position.r1() == other.position)
            || (self.position.r2() == other.position)
            || (self.position.r3() == other.position)
            || (self.position.s() == other.position)
            || (self.position.sr1() == other.position)
            || (self.position.sr2() == other.position)
            || (self.position.sr3() == other.position)
    }
}

impl Eq for TicTacToePositionD4Eq {}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct HashAndPosition(u64, TicTacToePosition);

impl Hash for TicTacToePositionD4Eq {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let hashes = [
            self.position.clone(),
            self.position.r1(),
            self.position.r2(),
            self.position.r3(),
            self.position.s(),
            self.position.sr1(),
            self.position.sr2(),
            self.position.sr3(),
        ]
        .map(|p| {
            let mut experiment_hasher = DefaultHasher::new();
            p.hash(&mut experiment_hasher);
            HashAndPosition(experiment_hasher.finish(), p)
        });

        let HashAndPosition(_, min_position) =
            hashes.iter().min().expect("hashes should not be empty");

        min_position.hash(state);
    }
}

impl Position<TicTacToeMove, TicTacToePrimitiveValue> for TicTacToePositionD4Eq {
    fn do_move(&self, mov: TicTacToeMove) -> TicTacToePositionD4Eq {
        TicTacToePositionD4Eq {
            position: self.position.do_move(mov),
        }
    }

    fn generate_moves(&self) -> Vec<TicTacToeMove> {
        self.position.generate_moves()
    }

    fn primitive_value(&self) -> TicTacToePrimitiveValue {
        self.position.primitive_value()
    }
}
