use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};

use crate::{
    games::order_and_chaos::{OrderAndChaosMove, OrderAndChaosPrimitiveValue},
    solver::{Position, PositionKey},
};

use super::OrderAndChaosPosition;

pub struct OrderAndChaosPositionVEq {
    pub position: OrderAndChaosPosition,
}

impl PositionKey for OrderAndChaosPositionVEq {}

impl PartialEq for OrderAndChaosPositionVEq {
    fn eq(&self, other: &Self) -> bool {
        (self.position == other.position)
            || (self.position.v1() == other.position)
            || (self.position.v2() == other.position)
            || (self.position.v3() == other.position)
    }
}

impl Eq for OrderAndChaosPositionVEq {}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct HashAndPosition(u64, OrderAndChaosPosition);

impl Hash for OrderAndChaosPositionVEq {
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

impl Position<OrderAndChaosMove, OrderAndChaosPrimitiveValue> for OrderAndChaosPositionVEq {
    fn do_move(&self, mov: OrderAndChaosMove) -> OrderAndChaosPositionVEq {
        OrderAndChaosPositionVEq {
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
    fn reflect_along_x(&self) -> Self {
        let mut board = self.board.clone();

        board.reverse();

        OrderAndChaosPosition {
            board,
            player: self.player,
        }
    }

    fn reflect_along_y(&self) -> Self {
        let mut board = self.board.clone();

        board.iter_mut().for_each(|row| row.reverse());

        OrderAndChaosPosition {
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
