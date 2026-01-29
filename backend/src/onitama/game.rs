use std::fmt::Display;

use rand::seq::SliceRandom;
use thiserror::Error;

use crate::onitama::{
    Color,
    board::{Board, coord_in_bounds},
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
            Color::Red => (Some(card_decides_player), None),
            Color::Blue => (None, Some(card_decides_player)),
        };

        Game {
            board: Board::new(),
            red_incoming,
            blue_incoming,
            red_cards: [deck.pop().unwrap(), deck.pop().unwrap()],
            blue_cards: [deck.pop().unwrap(), deck.pop().unwrap()],
        }
    }

    pub fn current_player(&self) -> Color {
        if let Some(_) = self.red_incoming {
            Color::Red
        } else {
            Color::Blue
        }
    }

    pub fn act(&mut self, action: &Action) -> Result<(), ActError> {
        if !coord_in_bounds(action.from) {
            return Err(ActError::FromPositionOutOfBounds {
                y: action.from.0,
                x: action.from.1,
            });
        }
        if !coord_in_bounds(action.to) {
            return Err(ActError::ToPositionOutOfBounds {
                y: action.to.0,
                x: action.to.1,
            });
        }

        let active_player_color = self.current_player();

        let active_cards = match active_player_color {
            Color::Red => self.red_cards,
            Color::Blue => self.blue_cards,
        };

        if !active_cards.contains(&action.card) {
            return Err(ActError::ActivePlayerDoesntHaveCard {
                active_player_color,
                card: action.card,
            });
        }

        match self.board.grid[action.from.0 as usize][action.from.1 as usize] {
            super::board::Cell::Empty => {
                return Err(ActError::ActivePlayerDoesntHavePieceAtFromPosition {
                    active_player_color,
                    y: action.from.0,
                    x: action.from.1,
                });
            }
            super::board::Cell::Taken(piece) => {
                if piece.color != active_player_color {
                    return Err(ActError::ActivePlayerDoesntHavePieceAtFromPosition {
                        active_player_color,
                        y: action.from.0,
                        x: action.from.1,
                    });
                }
            }
        }

        Ok(())
    }
}

#[derive(Error, Debug)]
pub enum ActError {
    #[error("the `from` coordinate ({y}, {x}) is out of bounds")]
    FromPositionOutOfBounds { y: i8, x: i8 },
    #[error("the `to` coordinate ({y}, {x}) is out of bounds")]
    ToPositionOutOfBounds { y: i8, x: i8 },
    #[error("the active player ({active_player_color}) doesn't have card ({card})")]
    ActivePlayerDoesntHaveCard {
        active_player_color: Color,
        card: CardId,
    },
    #[error(
        "the active player ({active_player_color}) doesn't have a piece at position ({y}, {x})"
    )]
    ActivePlayerDoesntHavePieceAtFromPosition {
        active_player_color: Color,
        y: i8,
        x: i8,
    },
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Action {
    from: (i8, i8),
    to: (i8, i8),
    card: CardId,
}
