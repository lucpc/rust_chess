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

impl King {
    pub fn new(color: Color) -> Self {
        Self { color, move_count: 0 }
    }
}

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
    fn increase_move_count(&mut self) { self.move_count += 1 }
    fn decrease_move_count(&mut self) { self.move_count -= 1 }
    fn box_clone(&self) -> Box<dyn Piece> { Box::new(self.clone()) }

    fn possible_moves(&self, board: &Board, pos: Position, game: &ChessMatch)
        -> Vec<Vec<bool>>
    {
        let mut mat = vec![vec![false; board.cols]; board.rows];
        let deltas = [
            (-1, 0), (1, 0), (0, -1), (0, 1),
            (-1,-1), (-1, 1), (1,-1), (1, 1)
        ];

        for (dr, dc) in deltas {
            let r = pos.row as isize + dr;
            let c = pos.col as isize + dc;

            if r >= 0 && c >= 0 && r < board.rows as isize && c < board.cols as isize {
                let p = Position::new(r as usize, c as usize);
                if !board.there_is_a_piece(p) || self.is_there_opponent_piece(p, board) {
                    mat[p.row][p.col] = true;
                }
            }
        }

        // castling
        if self.move_count == 0 && !game.check {
            let _opponent = if self.color == Color::White { Color::Black } else { Color::White };

            // king-side
            let rook_pos = Position::new(pos.row, pos.col + 3);

            if let Some(rook) = board.piece(rook_pos) {
                if rook.color() == self.color && rook.move_count() == 0 {
                    let p1 = Position::new(pos.row, pos.col + 1);
                    let p2 = Position::new(pos.row, pos.col + 2);

                    if !board.there_is_a_piece(p1)
                        && !board.there_is_a_piece(p2)
                    {
                        mat[p2.row][p2.col] = true;
                    }
                }
            }

            // queen-side
            let rook_pos_q = Position::new(pos.row, pos.col - 4);

            if let Some(rook) = board.piece(rook_pos_q) {
                if rook.color() == self.color && rook.move_count() == 0 {
                    let p1 = Position::new(pos.row, pos.col - 1);
                    let p2 = Position::new(pos.row, pos.col - 2);
                    let p3 = Position::new(pos.row, pos.col - 3);

                    if !board.there_is_a_piece(p1)
                        && !board.there_is_a_piece(p2)
                        && !board.there_is_a_piece(p3)
                    {
                        mat[p2.row][p2.col] = true;
                    }
                }
            }
        }

        mat
    }
}
