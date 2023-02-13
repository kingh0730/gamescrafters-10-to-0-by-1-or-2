use super::{GameResult, RecursiveValue};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct GameResultWithRemoteness {
    game_result: GameResult,
    remoteness: u32,
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
                .filter(
                    |GameResultWithRemoteness {
                         game_result,
                         remoteness,
                     }| *game_result == keep_game_result,
                )
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
                remoteness: inf,
            },
        }
    }
}
