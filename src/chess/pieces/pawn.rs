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

impl Pawn { pub fn new(color: Color) -> Self { Self { color, move_count: 0 } } }
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
    fn increase_move_count(&mut self) { self.move_count += 1; }
    fn decrease_move_count(&mut self) { self.move_count -= 1; }
    fn box_clone(&self) -> Box<dyn Piece> { Box::new(self.clone()) }

    fn possible_moves(&self, board: &Board, pos: Position, chess_match: &ChessMatch) -> Vec<Vec<bool>> {
        let mut mat = vec![vec![false; board.cols]; board.rows];
        let dir: isize = if self.color == Color::White { -1 } else { 1 };

        // 1 step forward
        let p1 = Position::new((pos.row as isize + dir) as usize, pos.col);
        if board.position_exists(p1) && !board.there_is_a_piece(p1) {
            mat[p1.row][p1.col] = true;
        }

        // 2 steps forward
        if self.move_count == 0 {
            let p2 = Position::new((pos.row as isize + 2 * dir) as usize, pos.col);
            if board.position_exists(p1) && !board.there_is_a_piece(p1) && board.position_exists(p2) && !board.there_is_a_piece(p2) {
                mat[p2.row][p2.col] = true;
            }
        }

        // Captures
        for &dc in &[-1, 1] {
            let new_col = pos.col as isize + dc;
            if new_col >= 0 && new_col < board.cols as isize {
                let p_capture = Position::new((pos.row as isize + dir) as usize, new_col as usize);
                if board.position_exists(p_capture) && self.is_there_opponent_piece(p_capture, board) {
                    mat[p_capture.row][p_capture.col] = true;
                }
            }
        }

        // En Passant
        if let Some(en_passant_pos) = chess_match.get_en_passant_vulnerable() {
            if (pos.row == 3 && self.color == Color::White) || (pos.row == 4 && self.color == Color::Black) {
                 for &dc in &[-1, 1] {
                    let adj_col = pos.col as isize + dc;
                    if adj_col >= 0 && adj_col < board.cols as isize {
                        let adj_pos = Position::new(pos.row, adj_col as usize);
                        if adj_pos == en_passant_pos {
                             let target_pos = Position::new((pos.row as isize + dir) as usize, adj_col as usize);
                             mat[target_pos.row][target_pos.col] = true;
                        }
                    }
                }
            }
        }

        mat
    }
}