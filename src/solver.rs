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

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::{GameResult, Position, PrimitiveValue, Solver};
    use crate::games::tic_tac_toe::{TicTacToePlayer, TicTacToePosition};

    #[test]
    fn it_works() {
        let mut solver = Solver::<_, _, _, GameResult>::new(HashMap::new());

        let result = solver.solve(TicTacToePosition {
            board: [[None, None, None], [None, None, None], [None, None, None]],
            player: TicTacToePlayer::X,
        });

        assert_eq!(result, GameResult::Tie);
    }

    #[test]
    fn tic_tac_toe_counts() {
        let mut solver = Solver::<_, _, _, GameResult>::new(HashMap::new());

        solver.solve(TicTacToePosition {
            board: [[None, None, None], [None, None, None], [None, None, None]],
            player: TicTacToePlayer::X,
        });

        let wins = solver
            .memoized_map
            .iter()
            .filter(|(_, &r)| r == GameResult::Win)
            .count();

        let loses = solver
            .memoized_map
            .iter()
            .filter(|(_, &r)| r == GameResult::Lose)
            .count();

        let ties = solver
            .memoized_map
            .iter()
            .filter(|(_, &r)| r == GameResult::Tie)
            .count();

        let total = solver.memoized_map.iter().count();

        let prim_wins = solver
            .memoized_map
            .iter()
            .filter(|(position, &r)| {
                r == GameResult::Win && position.primitive_value().is_primitive()
            })
            .count();

        let prim_loses = solver
            .memoized_map
            .iter()
            .filter(|(position, &r)| {
                r == GameResult::Lose && position.primitive_value().is_primitive()
            })
            .count();

        let prim_ties = solver
            .memoized_map
            .iter()
            .filter(|(position, &r)| {
                r == GameResult::Tie && position.primitive_value().is_primitive()
            })
            .count();

        let prim_total = solver
            .memoized_map
            .iter()
            .filter(|(position, _)| position.primitive_value().is_primitive())
            .count();

        assert_eq!(2836, wins);
        assert_eq!(1574, loses);
        assert_eq!(1068, ties);
        assert_eq!(5478, total);

        assert_eq!(0, prim_wins);
        assert_eq!(942, prim_loses);
        assert_eq!(16, prim_ties);
        assert_eq!(958, prim_total);

        println!("wins: {}", wins);
        println!("loses: {}", loses);
        println!("ties: {}", ties);
        println!("total: {}", total);

        println!("primitive wins: {}", prim_wins);
        println!("primitive loses: {}", prim_loses);
        println!("primitive ties: {}", prim_ties);
        println!("primitive total: {}", prim_total);
    }
}
