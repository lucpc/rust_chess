use crate::board::piece::Piece;
use crate::board::position::Position;
use crate::board::Board;
use crate::chess::{color::Color, ChessMatch};
use std::fmt;

#[derive(Clone)]
pub struct Bishop {
    color: Color,
    move_count: u32,
}

impl Bishop { pub fn new(color: Color) -> Self { Self { color, move_count: 0 } } }
impl fmt::Display for Bishop {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.color {
            Color::White => write!(f, "♗"),
            Color::Black => write!(f, "♝"),
        }
    }
}

impl Piece for Bishop {
    fn color(&self) -> Color { self.color }
    fn move_count(&self) -> u32 { self.move_count }
    fn increase_move_count(&mut self) { self.move_count += 1; }
    fn decrease_move_count(&mut self) { self.move_count -= 1; }
    fn box_clone(&self) -> Box<dyn Piece> { Box::new(self.clone()) }

    fn possible_moves(&self, board: &Board, pos: Position, _: &ChessMatch) -> Vec<Vec<bool>> {
        let mut mat = vec![vec![false; board.cols]; board.rows];
        let deltas = [(-1, -1), (-1, 1), (1, -1), (1, 1)]; // NW, NE, SW, SE

        for (dr, dc) in deltas {
            let mut current_row = pos.row as isize;
            let mut current_col = pos.col as isize;
            loop {
                current_row += dr;
                current_col += dc;
                 if current_row < 0 || current_row >= board.rows as isize || current_col < 0 || current_col >= board.cols as isize {
                    break;
                }
                let p = Position::new(current_row as usize, current_col as usize);
                if !board.there_is_a_piece(p) {
                    mat[p.row][p.col] = true;
                } else {
                    if self.is_there_opponent_piece(p, board) {
                        mat[p.row][p.col] = true;
                    }
                    break;
                }
            }
        }
        mat
    }
}