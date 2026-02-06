# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Build & Test Commands

```bash
cargo build                          # build (debug)
cargo build --release                # build (release, with LTO)
cargo test                           # run all tests
cargo test <test_name>               # run a single test by name
cargo run                            # run the game binary
```

Rust edition 2024. No linter or formatter config exists; use `cargo fmt` and `cargo clippy` as needed.

## Architecture

This is a Rust implementation of the board game **Onitama** — a two-player abstract strategy game on a 5x5 grid.

### Module structure (`src/onitama/`)

- **`mod.rs`** — Defines `Color` enum (Red/Blue) shared across all modules.
- **`board.rs`** — `Board` with a `[[Cell; 5]; 5]` grid. `Cell` is either `Empty` or `Taken(Piece)`. Red starts at row 0, Blue at row 4. Kings are at column 2.
- **`piece.rs`** — `Piece` struct with `PieceType` (Pawn/King) and `Color`.
- **`card.rs`** — `Card` struct with name, moves (as `Vec<(i8, i8)>` offsets), auto-computed `rotated_moves`, and color. Moves use `(row, col)` / `(y, x)` convention.
- **`cards.rs`** — `CardId` enum for all 16 cards. `CARDS` static (`LazyLock<HashMap<CardId, Card>>`) holds card definitions. `CardId::get()` returns `&'static Card`. `ALL_CARD_IDS` array used for deck shuffling.
- **`coordinate.rs`** — `Coordinate` (u8, u8) with `TryFrom` conversions and bounds checking, `Offset` (i8, i8) with arithmetic ops. Board bounds are `0..5` on both axes. Note: `Coordinate`/`Offset` types are newer and not yet used everywhere (e.g., `game.rs` still uses raw `(i8, i8)` tuples).
- **`game.rs`** — `Game` struct manages board, card dealing (5 random cards from 16), turn tracking via incoming card slots, and move validation (`act` method). `Action` struct holds from/to coordinates and card used.

### Key patterns

- Card data is static and immutable, accessed via `CardId::get()` which looks up from the `LazyLock` map.
- Turn order is determined by which player has an incoming card (`red_incoming.is_some()` means Red's turn).
- The `act` method validates moves but doesn't yet execute piece movement on the board.
