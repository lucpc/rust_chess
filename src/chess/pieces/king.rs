use crate::board::piece::Piece;
use crate::board::position::Position;
use crate::board::Board;
use crate::chess::{color::Color, ChessMatch};
use std::fmt;

#[derive(Clone)]
pub struct King {
    color: Color,
    move_count: u32,
}

impl King { pub fn new(color: Color) -> Self { Self { color, move_count: 0 } } }
impl fmt::Display for King {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.color {
            Color::White => write!(f, "♔"),
            Color::Black => write!(f, "♚"),
        }
    }
}
impl Piece for King {
    fn color(&self) -> Color { self.color }
    fn move_count(&self) -> u32 { self.move_count }
    fn increase_move_count(&mut self) { self.move_count += 1; }
    fn decrease_move_count(&mut self) { self.move_count -= 1; }
    fn box_clone(&self) -> Box<dyn Piece + Send + Sync> { Box::new(self.clone()) }

    fn possible_moves(&self, board: &Board, pos: Position, chess_match: &ChessMatch) -> Vec<Vec<bool>> {
        let mut mat = vec![vec![false; board.cols]; board.rows];
        let deltas = [
            (-1, 0), (1, 0), (0, -1), (0, 1),
            (-1, -1), (-1, 1), (1, -1), (1, 1),
        ];

        for (dr, dc) in deltas {
            let new_row = pos.row as isize + dr;
            let new_col = pos.col as isize + dc;
             if new_row >= 0 && new_row < board.rows as isize && new_col >= 0 && new_col < board.cols as isize {
                let p = Position::new(new_row as usize, new_col as usize);
                if !board.there_is_a_piece(p) || self.is_there_opponent_piece(p, board) {
                    mat[p.row][p.col] = true;
                }
            }
        }

        // Castling
        if self.move_count == 0 && !chess_match.check {
            // Kingside
            let rook_pos1 = Position::new(pos.row, pos.col + 3);
            if let Some(piece) = board.piece(rook_pos1) {
                if piece.move_count() == 0 && piece.to_string() == "R" {
                    let p1 = Position::new(pos.row, pos.col + 1);
                    let p2 = Position::new(pos.row, pos.col + 2);
                    if !board.there_is_a_piece(p1) && !board.there_is_a_piece(p2) {
                        mat[pos.row][pos.col + 2] = true;
                    }
                }
            }
            // Queenside
            let rook_pos2 = Position::new(pos.row, pos.col - 4);
            if let Some(piece) = board.piece(rook_pos2) {
                if piece.move_count() == 0 && piece.to_string() == "R" {
                    let p1 = Position::new(pos.row, pos.col - 1);
                    let p2 = Position::new(pos.row, pos.col - 2);
                    let p3 = Position::new(pos.row, pos.col - 3);
                     if !board.there_is_a_piece(p1) && !board.there_is_a_piece(p2) && !board.there_is_a_piece(p3) {
                        mat[pos.row][pos.col - 2] = true;
                    }
                }
            }
        }
        mat
    }
}