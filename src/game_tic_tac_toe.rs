use crate::solver::{GameResult, Move, Position, PrimitiveValue};

const LENGTH: usize = 3;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum TicTacToePlayer {
    X,
    O,
}

impl TicTacToePlayer {
    fn next_player(&self) -> TicTacToePlayer {
        match self {
            TicTacToePlayer::X => TicTacToePlayer::O,
            TicTacToePlayer::O => TicTacToePlayer::X,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
enum TicTacToePrimitiveValue {
    _Win,
    Lose,
    Tie,
    NotPrimitive,
}

#[derive(Debug)]
struct TicTacToeMove {
    x: usize,
    y: usize,
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct TicTacToePosition {
    board: [[Option<TicTacToePlayer>; LENGTH]; LENGTH],
    player: TicTacToePlayer,
}

impl Move for TicTacToeMove {}

impl PrimitiveValue for TicTacToePrimitiveValue {
    fn result(&self) -> Option<GameResult> {
        match self {
            TicTacToePrimitiveValue::_Win => Some(GameResult::Win),
            TicTacToePrimitiveValue::Lose => Some(GameResult::Lose),
            TicTacToePrimitiveValue::Tie => Some(GameResult::Tie),
            TicTacToePrimitiveValue::NotPrimitive => None,
        }
    }
}

impl Position for TicTacToePosition {
    type GameMove = TicTacToeMove;
    type GamePrimitiveValue = TicTacToePrimitiveValue;

    fn do_move(&self, mov: TicTacToeMove) -> TicTacToePosition {
        let mut board = self.board.clone();

        board[mov.x][mov.y] = Some(self.player);

        TicTacToePosition {
            board,
            player: self.player.next_player(),
        }
    }

    fn generate_moves(&self) -> Vec<TicTacToeMove> {
        let mut moves = vec![];

        self.board.iter().enumerate().for_each(|(i, line)| {
            line.iter().enumerate().for_each(|(j, cell)| {
                if let None = cell {
                    moves.push(TicTacToeMove { x: i, y: j });
                }
            })
        });

        moves
    }

    fn primitive_value(&self) -> TicTacToePrimitiveValue {
        let opponent = self.player.next_player();

        if (0..LENGTH).any(|i| (0..LENGTH).all(|j| self.board[i][j] == Some(opponent))) {
            return TicTacToePrimitiveValue::Lose;
        }

        if (0..LENGTH).any(|i| (0..LENGTH).all(|j| self.board[j][i] == Some(opponent))) {
            return TicTacToePrimitiveValue::Lose;
        }

        if (0..LENGTH).all(|i| self.board[i][i] == Some(opponent)) {
            return TicTacToePrimitiveValue::Lose;
        }

        if (0..LENGTH).all(|i| self.board[i][LENGTH - 1 - i] == Some(opponent)) {
            return TicTacToePrimitiveValue::Lose;
        }

        if (0..LENGTH).any(|i| (0..LENGTH).any(|j| self.board[i][j] == None)) {
            return TicTacToePrimitiveValue::NotPrimitive;
        }

        TicTacToePrimitiveValue::Tie
    }
}

#[cfg(test)]
mod tests {
    use super::{TicTacToePlayer, TicTacToePosition, TicTacToePrimitiveValue};
    use crate::solver::{GameResult, Position, Solver};
    use std::collections::HashMap;

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
    fn counts() {
        let mut solver = Solver::new(HashMap::new());

        solver.solve(TicTacToePosition {
            board: [[None, None, None], [None, None, None], [None, None, None]],
            player: TicTacToePlayer::X,
        });

        let wins = solver
            .memoized_results
            .iter()
            .filter(|(_, r)| **r == GameResult::Win)
            .count();

        let loses = solver
            .memoized_results
            .iter()
            .filter(|(_, r)| **r == GameResult::Lose)
            .count();

        let ties = solver
            .memoized_results
            .iter()
            .filter(|(_, r)| **r == GameResult::Tie)
            .count();

        let total = solver.memoized_results.iter().count();

        let prim_wins = solver
            .memoized_results
            .iter()
            .filter(|(p, r)| {
                **r == GameResult::Win
                    && p.primitive_value() != TicTacToePrimitiveValue::NotPrimitive
            })
            .count();

        let prim_loses = solver
            .memoized_results
            .iter()
            .filter(|(p, r)| {
                **r == GameResult::Lose
                    && p.primitive_value() != TicTacToePrimitiveValue::NotPrimitive
            })
            .count();

        let prim_ties = solver
            .memoized_results
            .iter()
            .filter(|(p, r)| {
                **r == GameResult::Tie
                    && p.primitive_value() != TicTacToePrimitiveValue::NotPrimitive
            })
            .count();

        let prim_total = solver
            .memoized_results
            .iter()
            .filter(|(p, _)| p.primitive_value() != TicTacToePrimitiveValue::NotPrimitive)
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
