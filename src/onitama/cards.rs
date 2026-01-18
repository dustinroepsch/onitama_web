use crate::onitama::Card;
use crate::onitama::Color;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref CARDS: [Card; 16] = [
        Card {
            name: "Tiger",
            mov: vec![(0, -1), (0, 2)],
            color: Color::Blue,
        },
        Card {
            name: "Dragon",
            mov: vec![(-1, -1), (-2, 1), (2, 1), (1, -1)],
            color: Color::Red,
        },
        Card {
            name: "Frog",
            mov: vec![(-2, 0), (-1, 1), (1, -1)],
            color: Color::Red,
        },
        Card {
            name: "Rabbit",
            mov: vec![(-1, -1), (1, 1), (2, 0)],
            color: Color::Blue,
        },
        Card {
            name: "Crab",
            mov: vec![(-2, 0), (0, 1), (2, 0)],
            color: Color::Blue,
        },
        Card {
            name: "Elephant",
            mov: vec![(-1, 0), (-1, 1), (1, 0), (1, 1)],
            color: Color::Red,
        },
        Card {
            name: "Goose",
            mov: vec![(-1, 0), (-1, 1), (1, 0), (1, -1)],
            color: Color::Blue,
        },
        Card {
            name: "Rooster",
            mov: vec![(-1, 0), (-1, -1), (1, 0), (1, 1)],
            color: Color::Red,
        },
        Card {
            name: "Monkey",
            mov: vec![(1, 1), (-1, -1), (-1, 1), (1, -1)],
            color: Color::Blue,
        },
        Card {
            name: "Mantis",
            mov: vec![(-1, 1), (0, -1), (1, 1)],
            color: Color::Red,
        },
        Card {
            name: "Horse",
            mov: vec![(-1, 0), (0, 1), (0, -1)],
            color: Color::Red,
        },
        Card {
            name: "Ox",
            mov: vec![(0, 1), (0, -1), (1, 0)],
            color: Color::Blue,
        },
        Card {
            name: "Crane",
            mov: vec![(0, 1), (-1, -1), (1, -1)],
            color: Color::Blue,
        },
        Card {
            name: "Boar",
            mov: vec![(-1, 0), (0, 1), (1, 0)],
            color: Color::Red,
        },
        Card {
            name: "Eel",
            mov: vec![(-1, 1), (-1, -1), (1, 0)],
            color: Color::Blue,
        },
        Card {
            name: "Cobra",
            mov: vec![(-1, 0), (1, 1), (1, -1)],
            color: Color::Red,
        },
    ];
}
