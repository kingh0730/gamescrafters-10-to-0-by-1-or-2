use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};

use crate::{
    games::tic_tac_toe_non_sq::{TicTacToeNonSqMove, TicTacToeNonSqPrimitiveValue, HEIGHT, WIDTH},
    solver::PositionKey,
};

use super::TicTacToeNonSqPosition;

pub struct TicTacToeNonSqPositionVEq {
    pub position: TicTacToeNonSqPosition,
}

impl PositionKey for TicTacToeNonSqPositionVEq {}

impl PartialEq for TicTacToeNonSqPositionVEq {
    fn eq(&self, other: &Self) -> bool {
        (self.position == other.position)
            || (self.position.v1() == other.position)
            || (self.position.v2() == other.position)
            || (self.position.v3() == other.position)
    }
}

impl Eq for TicTacToeNonSqPositionVEq {}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct HashAndPosition(u64, TicTacToeNonSqPosition);

impl Hash for TicTacToeNonSqPositionVEq {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let hashes = [
            self.position.clone(),
            self.position.v1(),
            self.position.v2(),
            self.position.v3(),
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

impl TicTacToeNonSqPosition {
    fn reflect_along_x(&self) -> Self {
        let mut board = self.board.clone();

        board.reverse();

        TicTacToeNonSqPosition {
            board,
            player: self.player,
        }
    }

    fn reflect_along_y(&self) -> Self {
        let mut board = self.board.clone();

        board.iter_mut().for_each(|row| row.reverse());

        TicTacToeNonSqPosition {
            board,
            player: self.player,
        }
    }

    fn v1(&self) -> Self {
        self.reflect_along_x()
    }

    fn v2(&self) -> Self {
        self.reflect_along_y()
    }

    fn v3(&self) -> Self {
        self.reflect_along_x().reflect_along_y()
    }
}
