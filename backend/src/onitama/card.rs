use core::fmt;
pub type Offset = (i8, i8);

pub type Moves = Vec<Offset>;

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
    pub name: &'static str,
    pub moves: Moves,
    pub rotated_moves: Moves,
    pub color: Color,
}

impl Card {
    pub fn new(name: &'static str, moves: Moves, color: Color) -> Self {
        let rotated_moves: Vec<_> = moves.iter().map(|(y, x)| (y * -1, x * -1)).collect();
        Card {
            name,
            moves: moves,
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
                if self.moves.contains(&(row, col)) {
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
                if self.rotated_moves.contains(&(row, col)) {
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
