use std::fmt::Display;

use crate::onitama::Color;
use crate::onitama::piece::Piece;
use crate::onitama::piece::PieceType::{King, Pawn};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Cell {
    Empty,
    Taken(Piece),
}

#[derive(Debug)]
pub struct Board {
    pub grid: [[Cell; 5]; 5],
}

impl Default for Board {
    fn default() -> Self {
        Self::new()
    }
}

impl Board {
    pub fn new() -> Self {
        let mut board = Board {
            grid: [[Cell::Empty; 5]; 5],
        };

        for i in 0..5 {
            board.grid[0][i] = Cell::Taken(Piece::new(Pawn, Color::Red));
            board.grid[4][i] = Cell::Taken(Piece::new(Pawn, Color::Blue));
        }

        board.grid[0][2] = Cell::Taken(Piece::new(King, Color::Red));
        board.grid[4][2] = Cell::Taken(Piece::new(King, Color::Blue));

        board
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in 0..5 {
            for col in 0..5 {
                write!(
                    f,
                    "{}",
                    match self.grid[row][col] {
                        Cell::Empty => "-",
                        Cell::Taken(piece) => match (piece.t, piece.color) {
                            (Pawn, Color::Red) => "♙",
                            (Pawn, Color::Blue) => "♟",
                            (King, Color::Red) => "♔",
                            (King, Color::Blue) => "♚",
                        },
                    }
                )?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

pub fn in_bounds(y: i8, x: i8) -> bool {
    (0..5).contains(&y) && (0..5).contains(&x)
}

pub fn coord_in_bounds((y, x): (i8, i8)) -> bool {
    in_bounds(y, x)
}
