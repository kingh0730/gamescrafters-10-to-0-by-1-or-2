mod player_move;
mod position;
mod primitive_value;
mod recursive_value;

use std::collections::HashMap;
use std::marker::PhantomData;

pub use self::player_move::PlayerMove;
pub use self::position::Position;
pub use self::primitive_value::PrimitiveValue;
pub use self::recursive_value::GameResult;
pub use self::recursive_value::GameResultWithRemoteness;
pub use self::recursive_value::RemotenessU32;
pub use self::recursive_value::ToRecursiveValue;

use self::recursive_value::RecursiveValue;

#[derive(Debug)]
pub struct Solver<P, M, PV, RV>
where
    P: Position<M, PV>,
    M: PlayerMove,
    PV: PrimitiveValue + ToRecursiveValue<RV>,
    RV: RecursiveValue,
{
    memoized_map: HashMap<P, RV>,

    _phantom_m: PhantomData<M>,
    _phantom_pv: PhantomData<PV>,
}

impl<P, M, PV, RV> Solver<P, M, PV, RV>
where
    P: Position<M, PV>,
    M: PlayerMove,
    PV: PrimitiveValue + ToRecursiveValue<RV>,
    RV: RecursiveValue,
{
    pub fn new(memoized_map: HashMap<P, RV>) -> Self {
        Self {
            memoized_map,

            _phantom_m: PhantomData,
            _phantom_pv: PhantomData,
        }
    }

    fn children(&self, position: &P) -> Vec<P> {
        position
            .generate_moves()
            .into_iter()
            .map(|mov| position.do_move(mov))
            .collect()
    }

    fn solve_not_memoized(&mut self, position: &P) -> RV {
        if let Some(result) = position.primitive_value().to_recursive_value() {
            return result;
        }

        let children_results = self
            .children(position)
            .into_iter()
            .map(|child| self.solve(child))
            .collect::<Vec<_>>();

        RV::recursion_step(&children_results)
    }

    pub fn solve(&mut self, position: P) -> RV {
        if let Some(result) = self.memoized_map.get(&position) {
            return result.clone();
        }

        let result = self.solve_not_memoized(&position);

        self.memoized_map.insert(position, result.clone());

        result
    }
}
