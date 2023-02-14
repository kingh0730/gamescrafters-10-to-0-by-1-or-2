use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};

use crate::solver::PositionGrpElem;

use super::TicTacToePosition;

struct TicTacToePositionGrpElem {
    position: TicTacToePosition,
}

impl PositionGrpElem for TicTacToePositionGrpElem {}

impl PartialEq for TicTacToePositionGrpElem {
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

impl Eq for TicTacToePositionGrpElem {}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct HashAndPosition(u64, TicTacToePosition);

impl Hash for TicTacToePositionGrpElem {
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
