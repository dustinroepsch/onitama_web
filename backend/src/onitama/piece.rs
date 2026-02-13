use serde::{Deserialize, Serialize};

use crate::onitama::Color;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PieceType {
    Pawn,
    King,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Piece {
    pub t: PieceType,
    pub color: Color,
}

impl Piece {
    pub fn new(t: PieceType, color: Color) -> Self {
        Piece { t, color }
    }
}
