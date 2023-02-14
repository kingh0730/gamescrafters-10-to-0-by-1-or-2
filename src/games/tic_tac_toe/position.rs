use crate::solver::{Position, PositionGrpElem};

use super::{TicTacToeMove, TicTacToePlayer, TicTacToePrimitiveValue, LENGTH};

mod position_grp_elem;

#[derive(Debug, PartialEq, Eq, Hash)]
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
