# Tic Tac Toe in Rust

A command-line implementation of Tic Tac Toe using object-oriented programming principles in Rust.

## Features

- Multiplayer support (2-4 players)
- Human vs Human, Human vs AI, or AI vs AI gameplay
- Adjustable board size (3-10)
- Multiple AI difficulty levels (Easy, Medium, Hard)
- Unique emoji symbols for each player
- Clean command-line interface with aligned grid
- Object-oriented design with modular components

## Installation

1. Make sure you have Rust and Cargo installed. If not, install from [rustup.rs](https://rustup.rs)
2. Clone this repository:
```bash
git clone https://github.com/vikyw89/rust-tic-tac-toe.git
cd rust-tic-tac-toe
```
3. Build the project:
```bash
cargo build --release
```

## Usage

Run the game:
```bash
cargo run
```

The game will prompt you for:
1. Board size (3-10, default: 3)
2. Number of players (2-4, default: 2)
3. For each player:
   - Type (Human or AI)
   - Name (for human players)
   - Difficulty level (for AI players)

When prompted for a move, enter the row and column numbers (0-based) separated by a space. For example:
```
Enter your move (row col): 1 1  # This will place your symbol in the center
```

## Project Structure

The project follows a flat directory structure with modular design, separating the library components from executable recipes.

### Library Components (`src/`)
Core game logic and components are organized as a library:

- `lib.rs` - Library root and public exports
- `game.rs` - Game flow and state management
- `game_board.rs` - Board implementation and move validation
- `player.rs` - Player traits and implementations
- `ai.rs` - AI player logic and difficulty levels
- `ui.rs` - User interface components
- `types.rs` - Shared types and enums

### Recipes (`recipes/`)
Executable examples and implementations:

- `tic_tac_toe.rs` - Main game executable with CLI interface

### Running the Game

```bash
# Run the main game
cargo run

# Run with release optimizations
cargo run --release
```

## Enums and Types

```rust
pub struct Symbol(char);  // Holds emoji character
pub enum GameStatus { InProgress, Win(String), Draw }
pub enum GameResult { Win, Loss, Draw }
pub enum Difficulty { Easy, Medium, Hard }
```

## Error Handling

```rust
pub enum GameError {
    InvalidMove,
    OutOfTurn,
    GameOver,
    MaxPlayersReached
}

pub enum PlayerError {
    NotFound,
    InvalidData
}
```

## Dependencies

- `rand = "0.8.5"`: For randomizing first player, AI moves, and player symbols

## Contributing

1. Fork the repository
2. Create a feature branch
3. Commit your changes
4. Push to the branch
5. Create a Pull Request

## License

MIT License
