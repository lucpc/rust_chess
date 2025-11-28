use super::position::Position;
use crate::board::Board;
use crate::chess::{color::Color, ChessMatch};
use std::fmt;

pub trait Piece: fmt::Display + Send + Sync {
    fn color(&self) -> Color;
    fn move_count(&self) -> u32;
    fn increase_move_count(&mut self);
    fn decrease_move_count(&mut self);

    fn possible_moves(&self, board: &Board, position: Position, chess_match: &ChessMatch) -> Vec<Vec<bool>>;
    
    fn is_there_opponent_piece(&self, position: Position, board: &Board) -> bool {
        board.piece(position).map_or(false, |p| p.color() != self.color())
    }

    fn box_clone(&self) -> Box<dyn Piece + Send + Sync>;
}

impl Clone for Box<dyn Piece + Send + Sync> {
    fn clone(&self) -> Self {
        self.box_clone()
    }
}