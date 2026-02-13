use std::fmt;

use serde::{Deserialize, Serialize};

pub mod board;
pub mod card;
pub mod cards;
pub mod coordinate;
pub mod game;
pub mod piece;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Color {
    Red,
    Blue,
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
