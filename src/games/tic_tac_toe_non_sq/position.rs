use crate::solver::{Position, PositionKey};

use super::{TicTacToeNonSqMove, TicTacToeNonSqPrimitiveValue, HEIGHT, WIDTH};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub enum TicTacToeNonSqPlayer {
    X,
    O,
}

impl TicTacToeNonSqPlayer {
    fn next_player(&self) -> TicTacToeNonSqPlayer {
        match self {
            TicTacToeNonSqPlayer::X => TicTacToeNonSqPlayer::O,
            TicTacToeNonSqPlayer::O => TicTacToeNonSqPlayer::X,
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
pub struct TicTacToePosition {
    pub board: [[Option<TicTacToeNonSqPlayer>; WIDTH]; HEIGHT],
    pub player: TicTacToeNonSqPlayer,
}

impl PositionKey for TicTacToePosition {}

impl Position<TicTacToeNonSqMove, TicTacToeNonSqPrimitiveValue> for TicTacToePosition {
    fn do_move(&self, mov: TicTacToeNonSqMove) -> TicTacToePosition {
        let mut board = self.board.clone();

        board[mov.x][mov.y] = Some(self.player);

        TicTacToePosition {
            board,
            player: self.player.next_player(),
        }
    }

    fn generate_moves(&self) -> Vec<TicTacToeNonSqMove> {
        let mut moves = vec![];

        self.board.iter().enumerate().for_each(|(i, line)| {
            line.iter().enumerate().for_each(|(j, cell)| {
                if let None = cell {
                    moves.push(TicTacToeNonSqMove { x: i, y: j });
                }
            })
        });

        moves
    }

    fn primitive_value(&self) -> TicTacToeNonSqPrimitiveValue {
        TicTacToeNonSqPrimitiveValue::Tie
    }
}
