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
