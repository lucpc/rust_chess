use crate::board::position::Position;
use crate::error::ChessError;
use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone, Copy)]
pub struct ChessPosition {
    pub col: char,
    pub row: u8,
}

impl ChessPosition {
    pub fn new(col: char, row: u8) -> Result<Self, ChessError> {
        if !('a'..='h').contains(&col) || !(1..=8).contains(&row) {
            return Err(ChessError("Error instantiating ChessPosition. Valid values are from a1 to h8.".to_string()));
        }
        Ok(ChessPosition { col, row })
    }

    pub fn to_position(&self) -> Position {
        Position {
            row: 8 - self.row as usize,
            col: (self.col as u8 - b'a') as usize,
        }
    }

    pub fn from_position(position: Position) -> Self {
        let col = (b'a' + position.col as u8) as char;
        let row = (8 - position.row) as u8;
        Self { col, row }
    }
}

impl FromStr for ChessPosition {
    type Err = ChessError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();
        if s.len() != 2 {
            return Err(ChessError("Invalid input. Expected format is like 'a1'.".to_string()));
        }
        let col = s.chars().next().unwrap();
        let row_char = s.chars().nth(1).unwrap();
        // let row = row_char.to_digit(10).ok_or_else(|| ChessError("Invalid row number.".to_string()))? as u8;
        let row = row_char.to_string().parse::<u8>().map_err(|_| ChessError("Invalid row number.".to_string()))?;
        ChessPosition::new(col, row)
    }
}

impl fmt::Display for ChessPosition {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", self.col, self.row)
    }
}