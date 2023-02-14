use super::RecursiveValue;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum GameResult {
    Win,
    Lose,
    Tie,
    Draw,
}

impl RecursiveValue for GameResult {
    fn recursion_step(children: &[Self]) -> Self {
        if children.iter().any(|&r| r == GameResult::Lose) {
            return GameResult::Win;
        }

        if children.iter().any(|&r| r == GameResult::Tie) {
            return GameResult::Tie;
        }

        if children.iter().any(|&r| r == GameResult::Draw) {
            return GameResult::Draw;
        }

        GameResult::Lose
    }
}

#[cfg(test)]
mod tests_with_games {
    use std::collections::HashMap;

    use super::GameResult;
    use crate::games::tic_tac_toe::{TicTacToePlayer, TicTacToePosition};
    use crate::solver::{Position, PrimitiveValue, Solver};

    #[test]
    fn it_works() {
        let mut solver = Solver::new(HashMap::new());

        let result = solver.solve(TicTacToePosition {
            board: [[None, None, None], [None, None, None], [None, None, None]],
            player: TicTacToePlayer::X,
        });

        assert_eq!(result, GameResult::Tie);
    }

    #[test]
    fn tic_tac_toe_counts() {
        let mut solver = Solver::new(HashMap::new());

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
