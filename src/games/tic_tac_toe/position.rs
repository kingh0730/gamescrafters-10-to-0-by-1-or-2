use crate::solver::{Position, PositionGrpElem};

use super::{TicTacToeMove, TicTacToePrimitiveValue, LENGTH};

mod position_grp_elem;

pub use position_grp_elem::TicTacToePositionGrpElem;

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

impl PositionGrpElem for TicTacToePosition {}

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

impl TicTacToePosition {
    fn reflect_along_x(&self) -> Self {
        let mut board = self.board.clone();

        board.reverse();

        TicTacToePosition {
            board,
            player: self.player,
        }
    }

    fn rotate_90(&self) -> Self {
        let mut board = [[None; LENGTH]; LENGTH];

        for i in 0..LENGTH {
            for j in 0..LENGTH {
                board[i][j] = self.board[LENGTH - j - 1][i];
            }
        }

        TicTacToePosition {
            board,
            player: self.player,
        }
    }

    fn r1(&self) -> Self {
        self.rotate_90()
    }

    fn r2(&self) -> Self {
        self.rotate_90().rotate_90()
    }

    fn r3(&self) -> Self {
        self.rotate_90().rotate_90().rotate_90()
    }

    fn s(&self) -> Self {
        self.reflect_along_x()
    }

    fn sr1(&self) -> Self {
        self.r1().reflect_along_x()
    }

    fn sr2(&self) -> Self {
        self.r2().reflect_along_x()
    }

    fn sr3(&self) -> Self {
        self.r3().reflect_along_x()
    }
}
