use std::fmt::Display;

use rand::seq::SliceRandom;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::onitama::{
    Color,
    board::{Board, Cell},
    cards::{ALL_CARD_IDS, CardId},
    coordinate::Coordinate,
};

#[derive(Debug, Serialize, Clone)]
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

#[derive(Debug)]
pub struct GameStartingState {
    card_decides_player: CardId,
    red_cards: [CardId; 2],
    blue_cards: [CardId; 2],
}

impl Game {
    pub fn new() -> Self {
        let mut deck: Vec<CardId> = ALL_CARD_IDS.to_vec();
        deck.shuffle(&mut rand::rng());
        Game::new_pre_determined(GameStartingState {
            card_decides_player: deck.pop().unwrap(),
            red_cards: [deck.pop().unwrap(), deck.pop().unwrap()],
            blue_cards: [deck.pop().unwrap(), deck.pop().unwrap()],
        })
    }

    pub fn new_pre_determined(
        GameStartingState {
            card_decides_player,
            red_cards,
            blue_cards,
        }: GameStartingState,
    ) -> Self {
        let (red_incoming, blue_incoming) = match card_decides_player.get().color {
            Color::Red => (Some(card_decides_player), None),
            Color::Blue => (None, Some(card_decides_player)),
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

        for offset in &action.card.get().moves {
            if action.from.try_add(offset) == Some(action.to) {
                *self.board.get_mut(action.to) = self.board.get(action.from).to_owned();
                *self.board.get_mut(action.from) = Cell::Empty;

                let (active_player_incoming, active_player_cards, next_player_incoming) =
                    match active_player_color {
                        Color::Red => (
                            &mut self.red_incoming,
                            &mut self.red_cards,
                            &mut self.blue_incoming,
                        ),
                        Color::Blue => (
                            &mut self.blue_incoming,
                            &mut self.blue_cards,
                            &mut self.red_incoming,
                        ),
                    };

                let location_of_card_just_played = active_player_cards.iter_mut().find(|c| **c == action.card).expect("We should be able to find the card just played in the active players cards");

                *location_of_card_just_played = active_player_incoming.expect("We just played a card, therefore we should have a new card available to replace it.");
                *active_player_incoming = None;
                *next_player_incoming = Some(action.card);

                return Ok(());
            }
        }

        Err(ActError::ChosenCardDoesntHaveMove {
            card_id: action.card,
            from: action.from,
            to: action.to,
        })
    }
}

#[derive(Error, Debug, PartialEq, Eq, Serialize)]
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
    #[error("The card {card_id} doesn't move from {from} to {to}")]
    ChosenCardDoesntHaveMove {
        card_id: CardId,
        from: Coordinate,
        to: Coordinate,
    },
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Action {
    pub from: Coordinate,
    pub to: Coordinate,
    pub card: CardId,
}

impl Action {
    pub fn new(from: Coordinate, to: Coordinate, card: CardId) -> Self {
        Self { from, to, card }
    }
}

#[cfg(test)]
mod tests {
    use crate::onitama::piece::{Piece, PieceType};

    use super::*;

    fn coord(y: u8, x: u8) -> Coordinate {
        (y, x).try_into().unwrap()
    }

    fn red_turn_game() -> Game {
        // Dragon is Red-colored, so red_incoming = Some → Red's turn
        Game::new_pre_determined(GameStartingState {
            card_decides_player: CardId::Dragon,
            red_cards: [CardId::Frog, CardId::Tiger],
            blue_cards: [CardId::Crab, CardId::Boar],
        })
    }

    fn blue_turn_game() -> Game {
        // Tiger is Blue-colored, so blue_incoming = Some → Blue's turn
        Game::new_pre_determined(GameStartingState {
            card_decides_player: CardId::Tiger,
            red_cards: [CardId::Frog, CardId::Rabbit],
            blue_cards: [CardId::Crab, CardId::Boar],
        })
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
        assert_eq!(game.current_player(), Color::Blue);
        assert_eq!(*game.board.get(coord(0, 0)), Cell::Empty);
        assert_eq!(
            *game.board.get(coord(1, 0)),
            Cell::Taken(Piece::new(PieceType::Pawn, Color::Red))
        );
    }

    #[test]
    fn act_with_valid_card_and_invalid_move() {
        let mut game = red_turn_game();
        // Red piece at (0, 0), using Tiger which Red holds, move to (1, 1), which Tiger doesn't allow you to do.
        let from = coord(0, 0);
        let to = coord(1, 1);
        let action = Action::new(from, to, CardId::Tiger);
        assert_eq!(
            game.act(&action),
            Err(ActError::ChosenCardDoesntHaveMove {
                card_id: CardId::Tiger,
                from,
                to
            })
        );
    }

    #[test]
    fn act_wrong_card() {
        let mut game = red_turn_game();
        // Rabbit belongs to Blue, not Red
        let action = Action::new(coord(0, 0), coord(1, 0), CardId::Rabbit);
        let result = game.act(&action);
        assert_eq!(
            result,
            Err(ActError::ActivePlayerDoesntHaveCard {
                active_player_color: Color::Red,
                card: CardId::Rabbit
            })
        );
    }

    #[test]
    fn act_empty_from() {
        let mut game = red_turn_game();
        // (2, 2) is an empty square in the middle of the board
        let from = coord(2, 2);
        let to = coord(3, 2);
        let action = Action::new(from, to, CardId::Tiger);
        let result = game.act(&action);
        assert_eq!(
            result,
            Err(ActError::ActivePlayerDoesntHavePieceAtFromPosition {
                active_player_color: Color::Red,
                y: 2,
                x: 2,
            })
        )
    }

    #[test]
    fn act_opponent_piece_at_from() {
        let mut game = red_turn_game();
        // (4, 0) has a Blue piece; Red can't move it
        let action = Action::new(coord(4, 0), coord(3, 0), CardId::Tiger);
        let result = game.act(&action);
        assert_eq!(
            result,
            Err(ActError::ActivePlayerDoesntHavePieceAtFromPosition {
                active_player_color: Color::Red,
                y: 4,
                x: 0,
            })
        )
    }
}
