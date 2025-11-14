# WARP.md

This file provides guidance to WARP (warp.dev) when working with code in this repository.

## Project Overview

A complete chess game implementation in Rust, playable via command line. This is a translation of a Java chess project, implementing standard chess rules including check, checkmate, castling, and en passant. The game is designed for two players in hot-seat mode.

## Common Commands

### Build and Run
```bash
# Compile and run the game
cargo run

# Build release version
cargo build --release

# Check code without building
cargo check
```

### Development
```bash
# Format code
cargo fmt

# Run clippy for linting
cargo clippy

# Clean build artifacts
cargo clean
```

## Architecture

### Module Structure

The codebase follows a layered architecture with clear separation of concerns:

**`board/`** - Generic chess board abstraction
- `Board`: 8x8 grid managing piece positions
- `Piece` trait: Defines piece behavior (movement, display)
- `Position`: Internal 0-indexed board coordinates (row, col)

**`chess/`** - Chess-specific game logic
- `ChessMatch`: Core game state manager, handles turn logic, check/checkmate detection, move validation, and special moves (castling, en passant, promotion)
- `ChessPosition`: External chess notation (e.g., "e4") that converts to internal `Position`
- `Color`: Player color enum (White/Black)
- `pieces/`: Individual piece implementations (King, Queen, Rook, Bishop, Knight, Pawn)

**`ui.rs`** - Terminal interface
- Renders board with Unicode chess symbols (♔♕♖♗♘♙ for white, ♚♛♜♝♞♟ for black)
- Highlights possible moves with blue background
- Displays captured pieces and game status

**`error.rs`** - Custom error type (`ChessError`) for game-related errors

**`main.rs`** - Game loop coordinating UI and game state

### Key Design Patterns

**Trait Objects for Polymorphism**: Pieces are stored as `Box<dyn Piece>` enabling different piece types to share movement validation logic while maintaining type-specific behavior.

**Move Validation Flow**:
1. `validate_source_position()` - Checks piece ownership and available moves
2. `validate_target_position()` - Verifies target is in possible moves
3. `make_move()` - Executes the move, handles captures and special moves
4. `test_check()` - Validates move doesn't leave player in check
5. `undo_move()` - Reverts invalid moves

**Check and Checkmate Logic**: 
- `test_check()`: Iterates opponent pieces to see if any can capture the king
- `test_check_mate()`: Tries all possible moves for the checked player; if all leave king in check, it's checkmate

**Special Moves**:
- **Castling**: Detected by king moving 2 squares horizontally; automatically moves corresponding rook
- **En Passant**: Tracked via `en_passant_vulnerable` field storing position of pawn that just moved 2 squares
- **Promotion**: Pawns reaching the opposite end auto-promote to Queen (hardcoded)

### State Management

`ChessMatch` maintains:
- `board`: Current piece positions
- `turn`: Move counter
- `current_player`: Active player color
- `check`/`check_mate`: Game state flags
- `en_passant_vulnerable`: Position eligible for en passant capture
- `pieces_on_board`: HashSet for efficient piece lookup
- `captured_pieces`: History of captured pieces

### Important Implementation Notes

**Piece Identification**: Pieces are identified by their Unicode display string (contains '♔'/'♚' for kings, '♟'/'♙' for pawns, etc.). This is used in move logic for castling and en passant detection. If piece display format changes, update string checks in `chess/mod.rs`.

**Coordinate Systems**: The codebase uses two coordinate systems:
- `Position`: 0-indexed (row 0 = top, col 0 = left) for internal board operations
- `ChessPosition`: Standard chess notation (a-h columns, 1-8 rows) for user input

**Move Count Tracking**: Each piece tracks `move_count` for castling eligibility and pawn double-move logic. Move/undo operations must maintain this correctly.

**Cloning Strategy**: `Piece` trait requires `box_clone()` method since trait objects can't derive Clone. Each piece implements this to enable move simulation for check validation.

## Dependencies

- `colored`: Terminal color output for piece and status display
- `clearscreen`: Cross-platform terminal clearing
- `lazy_static`: Used for static HashMap initialization (if needed for piece lookups)

## Game Flow

1. `ChessMatch::new()` initializes board with standard chess setup
2. Main loop displays board and prompts for source position
3. After valid source, displays possible moves (blue highlights) and prompts for target
4. Move validation checks ownership, legality, and check constraints
5. Special move logic executes automatically (castling, en passant, promotion)
6. Turn alternates until checkmate detected
7. Winner declared based on `current_player` when checkmate occurs

## Notes

- No formal test suite exists; validation is manual gameplay
- Pawn promotion is hardcoded to Queen (no choice prompt)
- No draw conditions implemented (stalemate, insufficient material, etc.)
- Game uses Portuguese comments in some areas (legacy from translation)
