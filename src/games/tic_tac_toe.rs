use crate::solver::{
    GameResult, PlayerMove, Position, PositionGrpElem, PrimitiveValue, ToRecursiveValue,
};

const LENGTH: usize = 3;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
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

#[derive(Debug, PartialEq, Eq)]
pub enum TicTacToePrimitiveValue {
    _Win,
    Lose,
    Tie,
    NotPrimitive,
}

#[derive(Debug)]
pub struct TicTacToeMove {
    x: usize,
    y: usize,
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct TicTacToePosition {
    pub board: [[Option<TicTacToePlayer>; LENGTH]; LENGTH],
    pub player: TicTacToePlayer,
}

impl PlayerMove for TicTacToeMove {}

impl PrimitiveValue for TicTacToePrimitiveValue {
    fn is_primitive(&self) -> bool {
        match self {
            TicTacToePrimitiveValue::NotPrimitive => false,
            _ => true,
        }
    }
}

impl ToRecursiveValue<GameResult> for TicTacToePrimitiveValue {
    fn to_recursive_value(&self) -> Option<GameResult> {
        match self {
            TicTacToePrimitiveValue::_Win => Some(GameResult::Win),
            TicTacToePrimitiveValue::Lose => Some(GameResult::Lose),
            TicTacToePrimitiveValue::Tie => Some(GameResult::Tie),
            TicTacToePrimitiveValue::NotPrimitive => None,
        }
    }
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

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::{TicTacToePlayer, TicTacToePosition};
    use crate::solver::{GameResult, Solver};

    #[test]
    fn it_works() {
        let mut solver = Solver::new(HashMap::new());

        let result = solver.solve(TicTacToePosition {
            board: [[None, None, None], [None, None, None], [None, None, None]],
            player: TicTacToePlayer::X,
        });

        assert_eq!(result, GameResult::Tie);
    }
}
