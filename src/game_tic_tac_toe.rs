use crate::{GameResult, Move, Position, PrimitiveValue};

const LENGTH: usize = 3;

#[derive(Debug, PartialEq, Eq, Hash)]
enum TicTacToePlayer {
    X,
    O,
}

#[derive(Debug)]
enum TicTacToePrimitiveValue {
    Win,
    Lose,
    Tie,
    NotPrimitive,
}

#[derive(Debug)]
enum TicTacToeMove {
    Take1,
    Take2,
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct TicTacToePosition {
    board: [[Option<TicTacToePlayer>; LENGTH]; LENGTH],
    player: TicTacToePlayer,
}

impl Move for TicTacToeMove {}
