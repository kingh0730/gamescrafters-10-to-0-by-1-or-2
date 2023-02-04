use crate::{GameResult, Move, Position, PrimitiveValue};

const LENGTH: usize = 3;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
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

#[derive(Debug)]
enum TicTacToePrimitiveValue {
    Win,
    Lose,
    Tie,
    NotPrimitive,
}

#[derive(Debug)]
struct TicTacToeMove {
    x: u32,
    y: u32,
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
            TicTacToePrimitiveValue::Win => Some(GameResult::Win),
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
        let board = self.board.clone();

        TicTacToePosition {
            board,
            player: self.player.next_player(),
        }
    }

    fn generate_moves(&self) -> Vec<TicTacToeMove> {
        vec![]
    }

    fn primitive_value(&self) -> TicTacToePrimitiveValue {
        TicTacToePrimitiveValue::NotPrimitive
    }
}
