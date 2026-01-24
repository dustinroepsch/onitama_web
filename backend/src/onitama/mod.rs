use core::fmt;

pub mod board;
pub mod cards;
pub mod game;
pub mod piece;

pub type Offset = (i8, i8);

pub type Move = Vec<Offset>;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Color {
    Red,
    Blue,
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Card {
    name: &'static str,
    mov: Move,
    color: Color,
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Name: {}", self.name)?;
        writeln!(f, "Color: {}", self.color)?;
        for row in -2..=2 {
            for col in -2..=2 {
                if self.mov.contains(&(col, row)) {
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
