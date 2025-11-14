pub mod piece;
pub mod position;

use self::piece::Piece;
use self::position::Position;
use crate::error::ChessError;

pub struct Board {
    pub rows: usize,
    pub cols: usize,
    pieces: Vec<Vec<Option<Box<dyn Piece>>>>,
}

impl Board {
    pub fn new(rows: usize, cols: usize) -> Result<Self, ChessError> {
        if rows < 1 || cols < 1 {
            return Err(ChessError("Error creating board: there must be at least 1 row and 1 column".to_string()));
        }
        let pieces = vec![vec![None; cols]; rows];
        Ok(Board { rows, cols, pieces })
    }

    pub fn piece(&self, position: Position) -> Option<&Box<dyn Piece>> {
        if !self.position_exists(position) {
            panic!("Position not on the board");
        }
        self.pieces[position.row][position.col].as_ref()
    }

    pub fn place_piece(&mut self, piece: Box<dyn Piece>, position: Position) -> Result<(), ChessError> {
        if self.there_is_a_piece(position) {
            return Err(ChessError(format!("There is already a piece on position {}", position)));
        }
        self.pieces[position.row][position.col] = Some(piece);
        Ok(())
    }

    pub fn remove_piece(&mut self, position: Position) -> Option<Box<dyn Piece>> {
        if !self.position_exists(position) {
            panic!("Position not on the board");
        }
        self.pieces[position.row][position.col].take()
    }

    pub fn position_exists(&self, position: Position) -> bool {
        position.row < self.rows && position.col < self.cols
    }

    pub fn there_is_a_piece(&self, position: Position) -> bool {
        if !self.position_exists(position) {
            panic!("Position not on the board");
        }
        self.piece(position).is_some()
    }
}