use crate::board::piece::Piece;
use crate::board::position::Position;
use crate::board::Board;
use crate::chess::{color::Color, ChessMatch};
use std::fmt;

#[derive(Clone)]
pub struct Pawn {
    color: Color,
    move_count: u32,
}

impl Pawn {
    pub fn new(color: Color) -> Self {
        Self { color, move_count: 0 }
    }
}

impl fmt::Display for Pawn {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.color {
            Color::White => write!(f, "♙"),
            Color::Black => write!(f, "♟"),
        }
    }
}

impl Piece for Pawn {
    fn color(&self) -> Color { self.color }
    fn move_count(&self) -> u32 { self.move_count }
    fn increase_move_count(&mut self) { self.move_count += 1 }
    fn decrease_move_count(&mut self) { self.move_count -= 1 }
    fn box_clone(&self) -> Box<dyn Piece> { Box::new(self.clone()) }

    fn possible_moves(&self, board: &Board, pos: Position, game: &ChessMatch)
        -> Vec<Vec<bool>>
    {
        let mut mat = vec![vec![false; board.cols]; board.rows];

        let dir: isize = if self.color == Color::White { -1 } else { 1 };

        // forward 1
        let f1 = (pos.row as isize + dir, pos.col as isize);
        if f1.0 >= 0 && f1.0 < board.rows as isize {
            let p = Position::new(f1.0 as usize, f1.1 as usize);
            if !board.there_is_a_piece(p) {
                mat[p.row][p.col] = true;
            }
        }

        // forward 2
        if self.move_count == 0 {
            let f2 = pos.row as isize + 2 * dir;
            if f1.0 >= 0 && f1.0 < board.rows as isize && f2 >= 0 && f2 < board.rows as isize {
                let p1 = Position::new(f1.0 as usize, pos.col);
                let p2 = Position::new(f2 as usize, pos.col);

                if !board.there_is_a_piece(p1) && !board.there_is_a_piece(p2) {
                    mat[p2.row][p2.col] = true;
                }
            }
        }

        // diagonal capture
        for dc in [-1, 1] {
            let r = pos.row as isize + dir;
            let c = pos.col as isize + dc;

            if r >= 0 && r < board.rows as isize && c >= 0 && c < board.cols as isize {
                let p = Position::new(r as usize, c as usize);

                if self.is_there_opponent_piece(p, board) {
                    mat[p.row][p.col] = true;
                }
            }
        }

        // en passant
        if let Some(ep) = game.get_en_passant_vulnerable() {
            if pos.row == 3 && self.color == Color::White
                || pos.row == 4 && self.color == Color::Black
            {
                for dc in [-1, 1] {
                    let adj = pos.col as isize + dc;

                    if adj >= 0 && adj < board.cols as isize {
                        let adj_pos = Position::new(pos.row, adj as usize);

                        if adj_pos == ep {
                            let target =
                                Position::new((pos.row as isize + dir) as usize, adj as usize);
                            mat[target.row][target.col] = true;
                        }
                    }
                }
            }
        }

        mat
    }
}
