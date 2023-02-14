use super::{GameResult, RecursiveValue};

trait Rmt {
    fn inf() -> Self;
    fn is_inf(&self) -> bool;
    fn increment(&self) -> Self;
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum RmtU32 {
    Val(u32),
    Inf,
}

impl Rmt for RmtU32 {
    fn inf() -> Self {
        Self::Inf
    }

    fn is_inf(&self) -> bool {
        match self {
            Self::Val(_) => false,
            Self::Inf => true,
        }
    }

    fn increment(&self) -> Self {
        match self {
            Self::Val(v) => Self::Val(v + 1),
            Self::Inf => Self::Inf,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct GameResultWithRmt {
    pub game_result: GameResult,
    pub rmt: RmtU32,
}

impl RecursiveValue for GameResultWithRmt {
    fn recursion_step(children: &[Self]) -> Self {
        let children_game_results = children
            .iter()
            .map(|GameResultWithRmt { game_result, .. }| *game_result)
            .collect::<Vec<_>>();

        let game_result = GameResult::recursion_step(&children_game_results);

        let filter_rmt = |keep_game_result| {
            children
                .iter()
                .filter(move |GameResultWithRmt { game_result, .. }| {
                    *game_result == keep_game_result
                })
                .map(|GameResultWithRmt { rmt, .. }| *rmt)
                .into_iter()
        };

        match game_result {
            GameResult::Win => GameResultWithRmt {
                game_result,
                rmt: filter_rmt(GameResult::Lose)
                    .min()
                    .expect("Non-primitive Win should have Lose child")
                    .increment(),
            },
            GameResult::Tie => GameResultWithRmt {
                game_result,
                rmt: filter_rmt(GameResult::Tie)
                    .min()
                    .expect("Non-primitive Tie should have Tie child")
                    .increment(),
            },
            GameResult::Lose => GameResultWithRmt {
                game_result,
                rmt: filter_rmt(GameResult::Win)
                    .max()
                    .expect("Non-primitive Lose should have Win child")
                    .increment(),
            },
            GameResult::Draw => GameResultWithRmt {
                game_result,
                rmt: RmtU32::Inf,
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::RmtU32;

    #[test]
    fn rmt_u32_ord() {
        let min = RmtU32::Val(u32::MIN);
        let zero = RmtU32::Val(0);
        let one = RmtU32::Val(1);
        let two = RmtU32::Val(2);
        let max_minus_one = RmtU32::Val(u32::MAX - 1);
        let max = RmtU32::Val(u32::MAX);
        let inf = RmtU32::Inf;

        assert_eq!(min, zero);
        assert_eq!(zero < one, true);
        assert_eq!(one < two, true);
        assert_eq!(two < max_minus_one, true);
        assert_eq!(max < max_minus_one, false);
        assert_eq!(inf < max, false);
        assert_eq!(zero < inf, true);
    }
}

#[cfg(test)]
mod tests_with_games {
    use std::collections::HashMap;

    use super::{GameResultWithRmt, RmtU32};
    use crate::games::take_10_to_0::TenToZeroPosition;
    use crate::games::tic_tac_toe::{TicTacToePlayer, TicTacToePosition, TicTacToePositionGrpElem};
    use crate::solver::{GameResult, Solver};

    #[test]
    fn it_works() {
        let mut solver = Solver::new(HashMap::<_, GameResultWithRmt>::new());

        let result = solver.solve(TenToZeroPosition {
            remaining_count: 10,
        });

        assert_eq!(
            result,
            GameResultWithRmt {
                game_result: GameResult::Win,
                rmt: RmtU32::Val(7),
            }
        );

        for i in (0..=10).rev() {
            let result = solver.solve(TenToZeroPosition { remaining_count: i });

            println!("{i}: {:?}", result);
        }
    }

    #[test]
    fn tic_tac_toe() {
        let mut solver = Solver::<_, _, _, GameResultWithRmt>::new(HashMap::new());

        let result = solver.solve(TicTacToePosition {
            board: [[None, None, None], [None, None, None], [None, None, None]],
            player: TicTacToePlayer::X,
        });

        assert_eq!(
            result,
            GameResultWithRmt {
                game_result: GameResult::Tie,
                rmt: RmtU32::Val(9),
            }
        );

        println!("{:#?}", result);
    }

    #[test]
    fn tic_tac_toe_counts() {
        let mut solver = Solver::<_, _, _, GameResultWithRmt>::new(HashMap::new());

        solver.solve(TicTacToePositionGrpElem {
            position: TicTacToePosition {
                board: [[None, None, None], [None, None, None], [None, None, None]],
                player: TicTacToePlayer::X,
            },
        });

        let wins = solver
            .memoized_map
            .iter()
            .filter(|(_, &r)| r.game_result == GameResult::Win)
            .count();

        let loses = solver
            .memoized_map
            .iter()
            .filter(|(_, &r)| r.game_result == GameResult::Lose)
            .count();

        let ties = solver
            .memoized_map
            .iter()
            .filter(|(_, &r)| r.game_result == GameResult::Tie)
            .count();

        let total = solver.memoized_map.iter().count();

        assert_eq!(2836, wins);
        assert_eq!(1574, loses);
        assert_eq!(1068, ties);
        assert_eq!(5478, total);

        println!("wins: {}", wins);
        println!("loses: {}", loses);
        println!("ties: {}", ties);
        println!("total: {}", total);
    }
}
