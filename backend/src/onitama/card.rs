use core::fmt;

use salvo::oapi::ToSchema;
use serde::Serialize;

use crate::onitama::Color;
use crate::onitama::coordinate::Offset;

pub type Moves = Vec<Offset>;

#[derive(Debug, Serialize, PartialEq, Eq, Clone, ToSchema)]
pub struct Card {
    pub name: String,
    pub moves: Moves,
    pub rotated_moves: Moves,
    pub color: Color,
}

impl Card {
    pub fn new(name: &str, moves: Moves, color: Color) -> Self {
        let rotated_moves: Vec<_> = moves.iter().map(|o| Offset { y: -o.y, x: -o.x }).collect();
        Card {
            name: name.to_string(),
            moves,
            //Todo: actually rotate the moves
            rotated_moves,
            color,
        }
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Name: {}", self.name)?;
        writeln!(f, "Color: {}", self.color)?;
        writeln!(f, "Moves: ")?;
        for row in -2..=2 {
            for col in -2..=2 {
                if self.moves.contains(&Offset { y: row, x: col }) {
                    write!(f, "X")?;
                } else if row == 0 && col == 0 {
                    write!(f, "o")?;
                } else {
                    write!(f, "-")?;
                }
            }
            writeln!(f)?;
        }
        writeln!(f, "Rotated Moves: ")?;
        for row in -2..=2 {
            for col in -2..=2 {
                if self.rotated_moves.contains(&Offset { y: row, x: col }) {
                    write!(f, "X")?;
                } else if row == 0 && col == 0 {
                    write!(f, "o")?;
                } else {
                    write!(f, "-")?;
                }
            }
            writeln!(f)?;
        }
        std::fmt::Result::Ok(())
    }
}
