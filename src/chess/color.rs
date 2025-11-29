use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Color {
    Black,
    White,
}

impl Color {
    pub fn is_white(&self) -> bool {
        matches!(self, Color::White)
    }

    pub fn is_black(&self) -> bool {
        matches!(self, Color::Black)
    }
}
