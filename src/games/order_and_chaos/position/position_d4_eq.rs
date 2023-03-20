use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};

use crate::games::order_and_chaos::{
    OrderAndChaosMove, OrderAndChaosPrimitiveValue, HEIGHT, WIDTH,
};
use crate::solver::{Position, PositionKey};

use super::OrderAndChaosPosition;

/// Should not use if WIDTH != HEIGHT
pub struct OrderAndChaosPositionD4Eq {
    pub position: OrderAndChaosPosition,
}

impl PositionKey for OrderAndChaosPositionD4Eq {}

impl PartialEq for OrderAndChaosPositionD4Eq {
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

impl Eq for OrderAndChaosPositionD4Eq {}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct HashAndPosition(u64, OrderAndChaosPosition);

impl Hash for OrderAndChaosPositionD4Eq {
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

impl Position<OrderAndChaosMove, OrderAndChaosPrimitiveValue> for OrderAndChaosPositionD4Eq {
    fn do_move(&self, mov: OrderAndChaosMove) -> OrderAndChaosPositionD4Eq {
        OrderAndChaosPositionD4Eq {
            position: self.position.do_move(mov),
        }
    }

    fn generate_moves(&self) -> Vec<OrderAndChaosMove> {
        self.position.generate_moves()
    }

    fn primitive_value(&self) -> OrderAndChaosPrimitiveValue {
        self.position.primitive_value()
    }
}

impl OrderAndChaosPosition {
    fn reflect_along_x_d4(&self) -> Self {
        let mut board = self.board.clone();

        board.reverse();

        OrderAndChaosPosition {
            board,
            player: self.player,
        }
    }

    /// Should not use if WIDTH != HEIGHT
    fn rotate_90(&self) -> Self {
        if WIDTH != HEIGHT {
            return self.clone();
        }

        let mut board = [[None; WIDTH]; HEIGHT];

        for i in 0..WIDTH {
            for j in 0..HEIGHT {
                board[i][j] = self.board[HEIGHT - j - 1][i];
            }
        }

        OrderAndChaosPosition {
            board,
            player: self.player,
        }
    }

    fn r1(&self) -> Self {
        self.rotate_90()
    }

    fn r2(&self) -> Self {
        self.rotate_90().rotate_90()
    }

    fn r3(&self) -> Self {
        self.rotate_90().rotate_90().rotate_90()
    }

    fn s(&self) -> Self {
        self.reflect_along_x_d4()
    }

    fn sr1(&self) -> Self {
        self.r1().reflect_along_x_d4()
    }

    fn sr2(&self) -> Self {
        self.r2().reflect_along_x_d4()
    }

    fn sr3(&self) -> Self {
        self.r3().reflect_along_x_d4()
    }
}
