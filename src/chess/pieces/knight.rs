use crate::board::piece::Piece;
use crate::board::position::Position;
use crate::board::Board;
use crate::chess::{color::Color, ChessMatch};
use std::fmt;

#[derive(Clone)]
pub struct Knight {
    color: Color,
    move_count: u32,
}

impl Knight { pub fn new(color: Color) -> Self { Self { color, move_count: 0 } } }
impl fmt::Display for Knight {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.color {
            Color::White => write!(f, "♘"),
            Color::Black => write!(f, "♞"),
        }
    }
}

impl Piece for Knight {
    fn color(&self) -> Color { self.color }
    fn move_count(&self) -> u32 { self.move_count }
    fn increase_move_count(&mut self) { self.move_count += 1; }
    fn decrease_move_count(&mut self) { self.move_count -= 1; }
    fn box_clone(&self) -> Box<dyn Piece> { Box::new(self.clone()) }

    fn possible_moves(&self, board: &Board, pos: Position, _: &ChessMatch) -> Vec<Vec<bool>> {
        let mut mat = vec![vec![false; board.cols]; board.rows];
        let moves = [
            (-1, -2), (-2, -1), (-2, 1), (-1, 2),
            (1, 2), (2, 1), (2, -1), (1, -2),
        ];

        for (dr, dc) in moves {
            let new_row = pos.row as isize + dr;
            let new_col = pos.col as isize + dc;

            if new_row >= 0 && new_row < board.rows as isize && new_col >= 0 && new_col < board.cols as isize {
                let p = Position::new(new_row as usize, new_col as usize);
                if !board.there_is_a_piece(p) || self.is_there_opponent_piece(p, board) {
                    mat[p.row][p.col] = true;
                }
            }
        }
        mat
    }
}