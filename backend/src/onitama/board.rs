use std::fmt::Display;

use crate::onitama::Color;
use crate::onitama::coordinate::Coordinate;
use crate::onitama::piece::Piece;
use crate::onitama::piece::PieceType::{King, Pawn};
use salvo::oapi::ToSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
pub enum Cell {
    Empty,
    Taken(Piece),
}

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct Board {
    grid: [[Cell; 5]; 5],
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

    pub fn get(&self, coord: Coordinate) -> &Cell {
        &self.grid[coord.y as usize][coord.x as usize]
    }

    pub fn get_mut(&mut self, coord: Coordinate) -> &mut Cell {
        &mut self.grid[coord.y as usize][coord.x as usize]
    }

    pub fn set(&mut self, coord: Coordinate, cell: Cell) {
        self.grid[coord.y as usize][coord.x as usize] = cell;
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

#[cfg(test)]
mod tests {
    use super::*;

    fn coord(y: u8, x: u8) -> Coordinate {
        Coordinate::try_from((y, x)).unwrap()
    }

    #[test]
    fn get_initial_red_king() {
        let board = Board::new();
        let cell = board.get(coord(0, 2));
        assert_eq!(*cell, Cell::Taken(Piece::new(King, Color::Red)));
    }

    #[test]
    fn get_initial_blue_king() {
        let board = Board::new();
        let cell = board.get(coord(4, 2));
        assert_eq!(*cell, Cell::Taken(Piece::new(King, Color::Blue)));
    }

    #[test]
    fn get_initial_red_pawn() {
        let board = Board::new();
        let cell = board.get(coord(0, 0));
        assert_eq!(*cell, Cell::Taken(Piece::new(Pawn, Color::Red)));
    }

    #[test]
    fn get_initial_empty_cell() {
        let board = Board::new();
        let cell = board.get(coord(2, 2));
        assert_eq!(*cell, Cell::Empty);
    }

    #[test]
    fn set_cell() {
        let mut board = Board::new();
        let target = coord(2, 2);
        board.set(target, Cell::Taken(Piece::new(Pawn, Color::Red)));
        assert_eq!(
            *board.get(target),
            Cell::Taken(Piece::new(Pawn, Color::Red))
        );
    }

    #[test]
    fn get_mut_cell() {
        let mut board = Board::new();
        let target = coord(0, 0);
        let cell = board.get_mut(target);
        *cell = Cell::Empty;
        assert_eq!(*board.get(target), Cell::Empty);
    }
}
