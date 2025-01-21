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

The project uses a flat directory structure with modular design.

### Core Components

#### Game (`game.rs`)
- Manages game flow and state
- Handles player turns and move validation
- Checks for win conditions and draws
- Supports multiple players

#### GameBoard (`game_board.rs`)
- Implements a variable-sized game board (n x n)
- Manages board state and move recording
- Checks for winning streaks in rows, columns, and diagonals

#### Player (`player.rs`)
- Represents players (human or AI)
- Manages player attributes (name, symbol)
- Handles player moves
- Supports unique emoji symbols

#### AI (`ai.rs`)
- Implements computer player logic
- Supports multiple difficulty levels (Easy, Medium, Hard)
- Uses minimax algorithm for Hard difficulty

#### UI (`ui.rs`)
- Handles command-line interface
- Displays game board with proper alignment
- Processes user input
- Shows game settings prompts

#### Types (`types.rs`)
- Defines core enums and types
- Implements error handling types
- Manages game status and results
- Defines difficulty levels

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
