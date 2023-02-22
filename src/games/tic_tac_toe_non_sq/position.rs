use crate::solver::{Position, PositionKey};

use super::{TicTacToeNonSqMove, TicTacToeNonSqPrimitiveValue, HEIGHT, K_IN_A_ROW, WIDTH};

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
pub struct TicTacToeNonSqPosition {
    pub board: [[Option<TicTacToeNonSqPlayer>; WIDTH]; HEIGHT],
    pub player: TicTacToeNonSqPlayer,
}

impl PositionKey for TicTacToeNonSqPosition {}

impl Position<TicTacToeNonSqMove, TicTacToeNonSqPrimitiveValue> for TicTacToeNonSqPosition {
    fn do_move(&self, mov: TicTacToeNonSqMove) -> TicTacToeNonSqPosition {
        let mut board = self.board.clone();

        board[mov.x][mov.y] = Some(self.player);

        TicTacToeNonSqPosition {
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
        let opponent = self.player.next_player();

        if (0..=(HEIGHT as i32 - K_IN_A_ROW as i32)).any(|offset| {
            (0..WIDTH)
                .any(|i| (0..HEIGHT).all(|j| self.board[j + offset as usize][i] == Some(opponent)))
        }) {
            return TicTacToeNonSqPrimitiveValue::Lose;
        }

        if (0..=(WIDTH as i32 - K_IN_A_ROW as i32)).any(|offset| {
            (0..HEIGHT)
                .any(|j| (0..WIDTH).all(|i| self.board[j][i + offset as usize] == Some(opponent)))
        }) {
            return TicTacToeNonSqPrimitiveValue::Lose;
        }

        if (0..WIDTH).any(|i| (0..HEIGHT).any(|j| self.board[j][i] == None)) {
            return TicTacToeNonSqPrimitiveValue::NotPrimitive;
        }

        TicTacToeNonSqPrimitiveValue::Tie
    }
}
