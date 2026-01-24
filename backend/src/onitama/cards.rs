use std::collections::HashMap;

use crate::onitama::Card;
use crate::onitama::Color;
use lazy_static::lazy_static;

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

lazy_static! {
    pub static ref CARDS: HashMap<CardId, Card> = HashMap::from([
        (
            CardId::Tiger,
            Card {
                name: "Tiger",
                mov: vec![(0, 1), (0, -2)],
                color: Color::Blue,
            }
        ),
        (
            CardId::Dragon,
            Card {
                name: "Dragon",
                mov: vec![(-1, 1), (-2, -1), (2, -1), (1, 1)],
                color: Color::Red,
            }
        ),
        (
            CardId::Frog,
            Card {
                name: "Frog",
                mov: vec![(-2, 0), (-1, -1), (1, 1)],
                color: Color::Red,
            }
        ),
        (
            CardId::Rabbit,
            Card {
                name: "Rabbit",
                mov: vec![(-1, 1), (1, -1), (2, 0)],
                color: Color::Blue,
            }
        ),
        (
            CardId::Crab,
            Card {
                name: "Crab",
                mov: vec![(-2, 0), (0, -1), (2, 0)],
                color: Color::Blue,
            }
        ),
        (
            CardId::Elephant,
            Card {
                name: "Elephant",
                mov: vec![(-1, 0), (-1, -1), (1, 0), (1, -1)],
                color: Color::Red,
            }
        ),
        (
            CardId::Goose,
            Card {
                name: "Goose",
                mov: vec![(-1, 0), (-1, -1), (1, 0), (1, 1)],
                color: Color::Blue,
            }
        ),
        (
            CardId::Rooster,
            Card {
                name: "Rooster",
                mov: vec![(-1, 0), (-1, 1), (1, 0), (1, -1)],
                color: Color::Red,
            }
        ),
        (
            CardId::Monkey,
            Card {
                name: "Monkey",
                mov: vec![(1, -1), (-1, 1), (-1, -1), (1, 1)],
                color: Color::Blue,
            }
        ),
        (
            CardId::Mantis,
            Card {
                name: "Mantis",
                mov: vec![(-1, -1), (0, 1), (1, -1)],
                color: Color::Red,
            }
        ),
        (
            CardId::Horse,
            Card {
                name: "Horse",
                mov: vec![(-1, 0), (0, -1), (0, 1)],
                color: Color::Red,
            }
        ),
        (
            CardId::Ox,
            Card {
                name: "Ox",
                mov: vec![(0, -1), (0, 1), (1, 0)],
                color: Color::Blue,
            }
        ),
        (
            CardId::Crane,
            Card {
                name: "Crane",
                mov: vec![(0, -1), (-1, 1), (1, 1)],
                color: Color::Blue,
            }
        ),
        (
            CardId::Boar,
            Card {
                name: "Boar",
                mov: vec![(-1, 0), (0, -1), (1, 0)],
                color: Color::Red,
            }
        ),
        (
            CardId::Eel,
            Card {
                name: "Eel",
                mov: vec![(-1, -1), (-1, 1), (1, 0)],
                color: Color::Blue,
            }
        ),
        (
            CardId::Cobra,
            Card {
                name: "Cobra",
                mov: vec![(-1, 0), (1, -1), (1, 1)],
                color: Color::Red,
            }
        ),
    ]);
}
