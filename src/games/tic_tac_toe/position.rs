use crate::solver::{Position, PositionKey};

use super::{TicTacToeMove, TicTacToePrimitiveValue, LENGTH};

mod position_d4_eq;

pub use position_d4_eq::TicTacToePositionD4Eq;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub enum TicTacToePlayer {
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

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
pub struct TicTacToePosition {
    pub board: [[Option<TicTacToePlayer>; LENGTH]; LENGTH],
    pub player: TicTacToePlayer,
}

impl PositionKey for TicTacToePosition {}

impl Position<TicTacToeMove, TicTacToePrimitiveValue> for TicTacToePosition {
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
