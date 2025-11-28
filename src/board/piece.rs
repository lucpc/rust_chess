use super::position::Position;
use crate::board::Board;

use crate::chess::{color::Color, ChessMatch};
use std::fmt;

pub trait Piece: fmt::Display + Send {
    fn color(&self) -> Color;
    fn move_count(&self) -> u32;
    fn increase_move_count(&mut self);
    fn decrease_move_count(&mut self);

    fn possible_moves(
        &self,
        board: &Board,
        position: Position,
        chess_match: &ChessMatch
    ) -> Vec<Vec<bool>>;

    /// Verifica se há uma peça oponente em uma posição.
    fn is_there_opponent_piece(&self, position: Position, board: &Board) -> bool {
        match board.piece(position) {
            Some(p) => p.color() != self.color(),
            None => false,
        }
    }

    fn box_clone(&self) -> Box<dyn Piece>;
}

impl Clone for Box<dyn Piece> {
    fn clone(&self) -> Self {
        self.box_clone()
    }
}
