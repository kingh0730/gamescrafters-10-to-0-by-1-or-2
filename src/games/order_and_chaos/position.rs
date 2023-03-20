use crate::solver::{Position, PositionKey};

use super::{
    OrderAndChaosMove, OrderAndChaosPiece, OrderAndChaosPrimitiveValue, HEIGHT, K_IN_A_ROW, WIDTH,
};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub enum OrderAndChaosPlayer {
    Order,
    Chaos,
}

impl OrderAndChaosPlayer {
    fn next_player(&self) -> OrderAndChaosPlayer {
        match self {
            OrderAndChaosPlayer::Order => OrderAndChaosPlayer::Chaos,
            OrderAndChaosPlayer::Chaos => OrderAndChaosPlayer::Order,
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
pub struct OrderAndChaosPosition {
    pub board: [[Option<OrderAndChaosPiece>; WIDTH]; HEIGHT],
    pub player: OrderAndChaosPlayer,
}

impl PositionKey for OrderAndChaosPosition {}

impl Position<OrderAndChaosMove, OrderAndChaosPrimitiveValue> for OrderAndChaosPosition {
    fn do_move(&self, mov: OrderAndChaosMove) -> OrderAndChaosPosition {
        let mut board = self.board.clone();

        board[mov.x][mov.y] = Some(mov.piece);

        OrderAndChaosPosition {
            board,
            player: self.player.next_player(),
        }
    }

    fn generate_moves(&self) -> Vec<OrderAndChaosMove> {
        let mut moves = vec![];

        self.board.iter().enumerate().for_each(|(i, line)| {
            line.iter().enumerate().for_each(|(j, cell)| {
                if let None = cell {
                    moves.push(OrderAndChaosMove {
                        piece: OrderAndChaosPiece::X,
                        x: i,
                        y: j,
                    });
                    moves.push(OrderAndChaosMove {
                        piece: OrderAndChaosPiece::O,
                        x: i,
                        y: j,
                    });
                }
            })
        });

        moves
    }

    fn primitive_value(&self) -> OrderAndChaosPrimitiveValue {
        let is_order = self.player == OrderAndChaosPlayer::Order;

        if has_k_in_a_row(&self.board, OrderAndChaosPiece::X) {
            return match is_order {
                true => OrderAndChaosPrimitiveValue::Win,
                false => OrderAndChaosPrimitiveValue::Lose,
            };
        }
        if has_k_in_a_row(&self.board, OrderAndChaosPiece::O) {
            return match is_order {
                true => OrderAndChaosPrimitiveValue::Win,
                false => OrderAndChaosPrimitiveValue::Lose,
            };
        }
        if has_no_space_left(&self.board) {
            return match is_order {
                true => OrderAndChaosPrimitiveValue::Lose,
                false => OrderAndChaosPrimitiveValue::Win,
            };
        }

        OrderAndChaosPrimitiveValue::NotPrimitive
    }
}

fn has_k_in_a_row(
    board: &[[Option<OrderAndChaosPiece>; WIDTH]; HEIGHT],
    piece: OrderAndChaosPiece,
) -> bool {
    if (0..=(HEIGHT as i32 - K_IN_A_ROW as i32)).any(|offset| {
        (0..WIDTH).any(|i| (0..K_IN_A_ROW).all(|j| board[j + offset as usize][i] == Some(piece)))
    }) {
        return true;
    }

    if (0..=(WIDTH as i32 - K_IN_A_ROW as i32)).any(|offset| {
        (0..HEIGHT).any(|j| (0..K_IN_A_ROW).all(|i| board[j][i + offset as usize] == Some(piece)))
    }) {
        return true;
    }

    if (0..=(HEIGHT as i32 - K_IN_A_ROW as i32)).any(|j_offset| {
        (0..=(WIDTH as i32 - K_IN_A_ROW as i32)).any(|i_offset| {
            (0..K_IN_A_ROW)
                .all(|x| board[x + j_offset as usize][x + i_offset as usize] == Some(piece))
                || (0..K_IN_A_ROW).all(|x| {
                    board[x + j_offset as usize][K_IN_A_ROW - 1 - x + i_offset as usize]
                        == Some(piece)
                })
        })
    }) {
        return true;
    }

    false
}

fn has_no_space_left(board: &[[Option<OrderAndChaosPiece>; WIDTH]; HEIGHT]) -> bool {
    if (0..WIDTH).all(|i| (0..HEIGHT).all(|j| board[j][i] != None)) {
        return true;
    }

    false
}
