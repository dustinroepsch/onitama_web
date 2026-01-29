use std::collections::HashMap;
use std::fmt::Display;
use std::sync::LazyLock;

use crate::onitama::Color;
use crate::onitama::card::Card;

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

pub static CARDS: LazyLock<HashMap<CardId, Card>> = LazyLock::new(|| {
    HashMap::from([
        (
            CardId::Tiger,
            Card::new("Tiger", vec![(1, 0), (-2, 0)], Color::Blue),
        ),
        (
            CardId::Dragon,
            Card::new(
                "Dragon",
                vec![(1, -1), (-1, -2), (-1, 2), (1, 1)],
                Color::Red,
            ),
        ),
        (
            CardId::Frog,
            Card::new("Frog", vec![(0, -2), (-1, -1), (1, 1)], Color::Red),
        ),
        (
            CardId::Rabbit,
            Card::new("Rabbit", vec![(1, -1), (-1, 1), (0, 2)], Color::Blue),
        ),
        (
            CardId::Crab,
            Card::new("Crab", vec![(0, -2), (-1, 0), (0, 2)], Color::Blue),
        ),
        (
            CardId::Elephant,
            Card::new(
                "Elephant",
                vec![(0, -1), (-1, -1), (0, 1), (-1, 1)],
                Color::Red,
            ),
        ),
        (
            CardId::Goose,
            Card::new(
                "Goose",
                vec![(0, -1), (-1, -1), (0, 1), (1, 1)],
                Color::Blue,
            ),
        ),
        (
            CardId::Rooster,
            Card::new(
                "Rooster",
                vec![(0, -1), (1, -1), (0, 1), (-1, 1)],
                Color::Red,
            ),
        ),
        (
            CardId::Monkey,
            Card::new(
                "Monkey",
                vec![(-1, 1), (1, -1), (-1, -1), (1, 1)],
                Color::Blue,
            ),
        ),
        (
            CardId::Mantis,
            Card::new("Mantis", vec![(-1, -1), (1, 0), (-1, 1)], Color::Red),
        ),
        (
            CardId::Horse,
            Card::new("Horse", vec![(0, -1), (-1, 0), (1, 0)], Color::Red),
        ),
        (
            CardId::Ox,
            Card::new("Ox", vec![(-1, 0), (1, 0), (0, 1)], Color::Blue),
        ),
        (
            CardId::Crane,
            Card::new("Crane", vec![(-1, 0), (1, -1), (1, 1)], Color::Blue),
        ),
        (
            CardId::Boar,
            Card::new("Boar", vec![(0, -1), (-1, 0), (0, 1)], Color::Red),
        ),
        (
            CardId::Eel,
            Card::new("Eel", vec![(-1, -1), (1, -1), (0, 1)], Color::Blue),
        ),
        (
            CardId::Cobra,
            Card::new("Cobra", vec![(0, -1), (-1, 1), (1, 1)], Color::Red),
        ),
    ])
});
