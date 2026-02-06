use std::fmt::Display;

use rand::seq::SliceRandom;
use thiserror::Error;

use crate::onitama::{
    Color,
    board::Board,
    cards::{ALL_CARD_IDS, CardId},
    coordinate::Coordinate,
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

    pub fn new_with_cards(
        red_cards: [CardId; 2],
        blue_cards: [CardId; 2],
        incoming: CardId,
    ) -> Self {
        let (red_incoming, blue_incoming) = match incoming.get().color {
            Color::Red => (Some(incoming), None),
            Color::Blue => (None, Some(incoming)),
        };

        Game {
            board: Board::new(),
            red_incoming,
            blue_incoming,
            red_cards,
            blue_cards,
        }
    }

    pub fn current_player(&self) -> Color {
        if self.red_incoming.is_some() {
            Color::Red
        } else {
            Color::Blue
        }
    }

    pub fn act(&mut self, action: &Action) -> Result<(), ActError> {
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

        match self.board.get(action.from) {
            super::board::Cell::Empty => {
                return Err(ActError::ActivePlayerDoesntHavePieceAtFromPosition {
                    active_player_color,
                    y: action.from.y,
                    x: action.from.x,
                });
            }
            super::board::Cell::Taken(piece) => {
                if piece.color != active_player_color {
                    return Err(ActError::ActivePlayerDoesntHavePieceAtFromPosition {
                        active_player_color,
                        y: action.from.y,
                        x: action.from.x,
                    });
                }
            }
        }

        Ok(())
    }
}

#[derive(Error, Debug)]
pub enum ActError {
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
        y: u8,
        x: u8,
    },
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Action {
    from: Coordinate,
    to: Coordinate,
    card: CardId,
}

impl Action {
    pub fn new(from: Coordinate, to: Coordinate, card: CardId) -> Self {
        Self { from, to, card }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn coord(y: u8, x: u8) -> Coordinate {
        (y, x).try_into().unwrap()
    }

    fn red_turn_game() -> Game {
        // Dragon is Red-colored, so red_incoming = Some → Red's turn
        Game::new_with_cards(
            [CardId::Tiger, CardId::Frog],
            [CardId::Rabbit, CardId::Crab],
            CardId::Dragon,
        )
    }

    fn blue_turn_game() -> Game {
        // Tiger is Blue-colored, so blue_incoming = Some → Blue's turn
        Game::new_with_cards(
            [CardId::Dragon, CardId::Frog],
            [CardId::Rabbit, CardId::Crab],
            CardId::Tiger,
        )
    }

    #[test]
    fn new_with_cards_red_goes_first() {
        let game = red_turn_game();
        assert_eq!(game.current_player(), Color::Red);
    }

    #[test]
    fn new_with_cards_blue_goes_first() {
        let game = blue_turn_game();
        assert_eq!(game.current_player(), Color::Blue);
    }

    #[test]
    fn act_with_valid_card_and_piece() {
        let mut game = red_turn_game();
        // Red piece at (0, 0), using Tiger which Red holds, move to (1, 0)
        let action = Action::new(coord(0, 0), coord(1, 0), CardId::Tiger);
        assert!(game.act(&action).is_ok());
    }

    #[test]
    fn act_wrong_card() {
        let mut game = red_turn_game();
        // Rabbit belongs to Blue, not Red
        let action = Action::new(coord(0, 0), coord(1, 0), CardId::Rabbit);
        let err = game.act(&action).unwrap_err();
        assert!(matches!(err, ActError::ActivePlayerDoesntHaveCard { .. }));
    }

    #[test]
    fn act_empty_from() {
        let mut game = red_turn_game();
        // (2, 2) is an empty square in the middle of the board
        let action = Action::new(coord(2, 2), coord(3, 2), CardId::Tiger);
        let err = game.act(&action).unwrap_err();
        assert!(matches!(
            err,
            ActError::ActivePlayerDoesntHavePieceAtFromPosition { .. }
        ));
    }

    #[test]
    fn act_opponent_piece_at_from() {
        let mut game = red_turn_game();
        // (4, 0) has a Blue piece; Red can't move it
        let action = Action::new(coord(4, 0), coord(3, 0), CardId::Tiger);
        let err = game.act(&action).unwrap_err();
        assert!(matches!(
            err,
            ActError::ActivePlayerDoesntHavePieceAtFromPosition { .. }
        ));
    }
}
