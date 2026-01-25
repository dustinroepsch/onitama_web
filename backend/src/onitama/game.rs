use std::fmt::Display;

use rand::seq::SliceRandom;

use crate::onitama::{
    board::Board,
    cards::{ALL_CARD_IDS, CardId},
};

#[derive(Debug)]
pub struct Game {
    board: Board,
    red_incoming: Option<CardId>,
    blue_incoming: Option<CardId>,
    red_cards: [CardId; 2],
    blue_cards: [CardId; 2],
}

impl Display for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", self.board)?;
        writeln!(f, "Red Card:{:?}", self.red_cards)?;
        for card in self.red_cards {
            writeln!(f, "{}", card.get())?;
        }
        writeln!(f, "Blue Card:{:?}", self.blue_cards)?;
        for card in self.blue_cards {
            writeln!(f, "{}", card.get())?;
        }
        writeln!(f, "Red Incoming:{:?}", self.red_incoming)?;
        if let Some(card) = self.red_incoming {
            writeln!(f, "{}", card.get())?;
        }
        writeln!(f, "Blue Incoming:{:?}", self.blue_incoming)?;
        if let Some(card) = self.blue_incoming {
            writeln!(f, "{}", card.get())?;
        }
        Ok(())
    }
}

impl Default for Game {
    fn default() -> Self {
        Self::new()
    }
}

impl Game {
    pub fn new() -> Self {
        let mut deck: Vec<CardId> = ALL_CARD_IDS.to_vec();
        deck.shuffle(&mut rand::rng());

        let card_decides_player = deck.pop().unwrap();

        let (red_incoming, blue_incoming) = match card_decides_player.get().color {
            super::Color::Red => (Some(card_decides_player), None),
            super::Color::Blue => (None, Some(card_decides_player)),
        };

        Game {
            board: Board::new(),
            red_incoming,
            blue_incoming,
            red_cards: [deck.pop().unwrap(), deck.pop().unwrap()],
            blue_cards: [deck.pop().unwrap(), deck.pop().unwrap()],
        }
    }
}
