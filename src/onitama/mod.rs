pub mod cards;

pub type Offset = (i8, i8);

pub type Move = Vec<Offset>;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Color {
    Red,
    Blue,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Card {
    name: &'static str,
    mov: Move,
    color: Color,
}
