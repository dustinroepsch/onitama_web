#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Color {
    Red,
    Blue,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum PieceType {
    Pawn,
    King,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Piece {
    pub t: PieceType,
    pub color: Color,
}

impl Piece {
    pub fn new(t: PieceType, color: Color) -> Self {
        Piece { t, color }
    }
}
