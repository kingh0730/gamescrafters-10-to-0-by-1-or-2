use super::{GameResult, RecursiveValue};

trait Remoteness {
    fn inf() -> Self;
    fn is_inf(&self) -> bool;
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum RemotenessU32 {
    Val(u32),
    Inf,
}

impl Remoteness for RemotenessU32 {
    fn inf() -> Self {
        Self::Inf
    }

    fn is_inf(&self) -> bool {
        match self {
            Self::Val(_) => false,
            Self::Inf => true,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct GameResultWithRemoteness {
    game_result: GameResult,
    remoteness: RemotenessU32,
}

impl RecursiveValue for GameResultWithRemoteness {
    fn recursion_step(children: &[Self]) -> Self {
        let children_game_results = children
            .iter()
            .map(|GameResultWithRemoteness { game_result, .. }| *game_result)
            .collect::<Vec<_>>();

        let game_result = GameResult::recursion_step(&children_game_results);

        let filter_remoteness = |keep_game_result| {
            children
                .iter()
                .filter(move |GameResultWithRemoteness { game_result, .. }| {
                    *game_result == keep_game_result
                })
                .map(|GameResultWithRemoteness { remoteness, .. }| *remoteness)
                .into_iter()
        };

        match game_result {
            GameResult::Win => GameResultWithRemoteness {
                game_result,
                remoteness: filter_remoteness(GameResult::Lose)
                    .min()
                    .expect("Non-primitive Win should have Lose child"),
            },
            GameResult::Tie => GameResultWithRemoteness {
                game_result,
                remoteness: filter_remoteness(GameResult::Tie)
                    .min()
                    .expect("Non-primitive Tie should have Tie child"),
            },
            GameResult::Lose => GameResultWithRemoteness {
                game_result,
                remoteness: filter_remoteness(GameResult::Win)
                    .max()
                    .expect("Non-primitive Lose should have Win child"),
            },
            GameResult::Draw => GameResultWithRemoteness {
                game_result,
                remoteness: RemotenessU32::Inf,
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::RemotenessU32;

    #[test]
    fn remoteness_u32_ord() {
        let min = RemotenessU32::Val(u32::MIN);
        let zero = RemotenessU32::Val(0);
        let one = RemotenessU32::Val(1);
        let two = RemotenessU32::Val(2);
        let max_minus_one = RemotenessU32::Val(u32::MAX - 1);
        let max = RemotenessU32::Val(u32::MAX);
        let inf = RemotenessU32::Inf;

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

    use super::{GameResultWithRemoteness, RemotenessU32};
    use crate::games::take_10_to_0::TenToZeroPosition;
    use crate::solver::{GameResult, Solver};

    #[test]
    fn it_works() {
        let mut solver = Solver::<_, _, _, GameResultWithRemoteness>::new(HashMap::new());

        let result = solver.solve(TenToZeroPosition {
            remaining_count: 10,
        });

        assert_eq!(
            result,
            GameResultWithRemoteness {
                game_result: GameResult::Tie,
                remoteness: RemotenessU32::Val(0),
            }
        );
    }
}
