use crate::board::piece::Piece;
use crate::board::position::Position;
use crate::board::Board;
use crate::chess::{color::Color, ChessMatch};
use std::fmt;

#[derive(Clone)]
pub struct Rook {
    color: Color,
    move_count: u32,
}

impl Rook {
    pub fn new(color: Color) -> Self {
        Self { color, move_count: 0 }
    }
}

impl fmt::Display for Rook {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.color {
            Color::White => write!(f, "♖"),
            Color::Black => write!(f, "♜"),
        }
    }
}

impl Piece for Rook {
    fn color(&self) -> Color { self.color }
    fn move_count(&self) -> u32 { self.move_count }
    fn increase_move_count(&mut self) { self.move_count += 1 }
    fn decrease_move_count(&mut self) { self.move_count -= 1 }
    fn box_clone(&self) -> Box<dyn Piece> { Box::new(self.clone()) }

    fn possible_moves(&self, board: &Board, pos: Position, _: &ChessMatch)
        -> Vec<Vec<bool>>
    {
        let mut mat = vec![vec![false; board.cols]; board.rows];
        let deltas = [(-1,0), (1,0), (0,-1), (0,1)];

        for (dr,dc) in deltas {
            let mut r = pos.row as isize;
            let mut c = pos.col as isize;

            loop {
                r += dr; c += dc;

                if r < 0 || c < 0 || r >= board.rows as isize || c >= board.cols as isize {
                    break;
                }

                let p = Position::new(r as usize, c as usize);

                if !board.there_is_a_piece(p) {
                    mat[p.row][p.col] = true;
                    continue;
                }

                if self.is_there_opponent_piece(p, board) {
                    mat[p.row][p.col] = true;
                }

                break;
            }
        }

        mat
    }
}
