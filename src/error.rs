use std::fmt;

#[derive(Debug)]
pub struct ChessError(pub String);

impl fmt::Display for ChessError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for ChessError {}