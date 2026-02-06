use std::collections::HashMap;
use std::fmt::Display;
use std::sync::LazyLock;

use crate::onitama::Color;
use crate::onitama::card::Card;
use crate::onitama::coordinate::Offset;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum CardId {
    Tiger,
    Dragon,
    Frog,
    Rabbit,
    Crab,
    Elephant,
    Goose,
    Rooster,
    Monkey,
    Mantis,
    Horse,
    Ox,
    Crane,
    Boar,
    Eel,
    Cobra,
}

impl Display for CardId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = self.get().name;
        write!(f, "{name}")
    }
}

pub static ALL_CARD_IDS: [CardId; 16] = [
    CardId::Tiger,
    CardId::Dragon,
    CardId::Frog,
    CardId::Rabbit,
    CardId::Crab,
    CardId::Elephant,
    CardId::Goose,
    CardId::Rooster,
    CardId::Monkey,
    CardId::Mantis,
    CardId::Horse,
    CardId::Ox,
    CardId::Crane,
    CardId::Boar,
    CardId::Eel,
    CardId::Cobra,
];

impl CardId {
    pub fn get(&self) -> &'static Card {
        CARDS.get(self).unwrap()
    }
}

fn o(y: i8, x: i8) -> Offset {
    Offset { y, x }
}

pub static CARDS: LazyLock<HashMap<CardId, Card>> = LazyLock::new(|| {
    HashMap::from([
        (
            CardId::Tiger,
            Card::new("Tiger", vec![o(1, 0), o(-2, 0)], Color::Blue),
        ),
        (
            CardId::Dragon,
            Card::new(
                "Dragon",
                vec![o(1, -1), o(-1, -2), o(-1, 2), o(1, 1)],
                Color::Red,
            ),
        ),
        (
            CardId::Frog,
            Card::new("Frog", vec![o(0, -2), o(-1, -1), o(1, 1)], Color::Red),
        ),
        (
            CardId::Rabbit,
            Card::new("Rabbit", vec![o(1, -1), o(-1, 1), o(0, 2)], Color::Blue),
        ),
        (
            CardId::Crab,
            Card::new("Crab", vec![o(0, -2), o(-1, 0), o(0, 2)], Color::Blue),
        ),
        (
            CardId::Elephant,
            Card::new(
                "Elephant",
                vec![o(0, -1), o(-1, -1), o(0, 1), o(-1, 1)],
                Color::Red,
            ),
        ),
        (
            CardId::Goose,
            Card::new(
                "Goose",
                vec![o(0, -1), o(-1, -1), o(0, 1), o(1, 1)],
                Color::Blue,
            ),
        ),
        (
            CardId::Rooster,
            Card::new(
                "Rooster",
                vec![o(0, -1), o(1, -1), o(0, 1), o(-1, 1)],
                Color::Red,
            ),
        ),
        (
            CardId::Monkey,
            Card::new(
                "Monkey",
                vec![o(-1, 1), o(1, -1), o(-1, -1), o(1, 1)],
                Color::Blue,
            ),
        ),
        (
            CardId::Mantis,
            Card::new("Mantis", vec![o(-1, -1), o(1, 0), o(-1, 1)], Color::Red),
        ),
        (
            CardId::Horse,
            Card::new("Horse", vec![o(0, -1), o(-1, 0), o(1, 0)], Color::Red),
        ),
        (
            CardId::Ox,
            Card::new("Ox", vec![o(-1, 0), o(1, 0), o(0, 1)], Color::Blue),
        ),
        (
            CardId::Crane,
            Card::new("Crane", vec![o(-1, 0), o(1, -1), o(1, 1)], Color::Blue),
        ),
        (
            CardId::Boar,
            Card::new("Boar", vec![o(0, -1), o(-1, 0), o(0, 1)], Color::Red),
        ),
        (
            CardId::Eel,
            Card::new("Eel", vec![o(-1, -1), o(1, -1), o(0, 1)], Color::Blue),
        ),
        (
            CardId::Cobra,
            Card::new("Cobra", vec![o(0, -1), o(-1, 1), o(1, 1)], Color::Red),
        ),
    ])
});
