use rand::seq::SliceRandom;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Symbol(pub char);

impl Symbol {
    const SYMBOLS: [char; 12] = [
        '🐱', '🐶', '🦊', '🐰', '🐼', '🐨', '🦁', '🐯', '🐸', '🦉', '🦄', '🐙',
    ];

    pub fn random_unique(used_symbols: &[Symbol]) -> Option<Symbol> {
        let available_symbols: Vec<_> = Self::SYMBOLS
            .iter()
            .copied()
            .filter(|&c| !used_symbols.contains(&Symbol(c)))
            .collect();

        available_symbols
            .choose(&mut rand::thread_rng())
            .map(|&c| Symbol(c))
    }

    pub fn next_symbol(current: &Symbol) -> Symbol {
        let symbols = ["🦁", "🐯", "🐨", "🐼", "🐸", "🦊", "🐱", "🐶"];
        let current_char = current.0;

        // Find current symbol's index
        let mut current_idx = 0;
        for (i, &symbol) in symbols.iter().enumerate() {
            if symbol.chars().next().unwrap() == current_char {
                current_idx = i;
                break;
            }
        }

        // Get next symbol (wrap around if at end)
        let next_idx = (current_idx + 1) % symbols.len();
        Symbol(symbols[next_idx].chars().next().unwrap())
    }
}

impl fmt::Display for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug)]
pub enum GameStatus {
    InProgress,
    Win(String), // Player name
    Draw,
}

#[derive(Debug)]
pub enum GameResult {
    Win,
    Loss,
    Draw,
}

#[derive(Debug, Clone, Copy)]
pub enum Difficulty {
    Easy,
    Medium,
    Hard,
}

#[derive(Debug)]
pub enum BoardError {
    InvalidPosition,
    CellOccupied,
    OutOfBounds,
}

impl fmt::Display for BoardError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BoardError::InvalidPosition => write!(f, "Invalid position"),
            BoardError::CellOccupied => write!(f, "Cell already occupied"),
            BoardError::OutOfBounds => write!(f, "Position is out of bounds"),
        }
    }
}

#[derive(Debug)]
pub enum GameError {
    InvalidMove,
    MaxPlayersReached,
    GameOver,
    OutOfTurn,
    BoardError(BoardError),
}

impl From<BoardError> for GameError {
    fn from(err: BoardError) -> Self {
        GameError::BoardError(err)
    }
}

impl fmt::Display for GameError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GameError::InvalidMove => write!(f, "Invalid move"),
            GameError::MaxPlayersReached => write!(f, "Maximum number of players reached"),
            GameError::GameOver => write!(f, "Game is over"),
            GameError::OutOfTurn => write!(f, "Not your turn"),
            GameError::BoardError(err) => write!(f, "Board error: {}", err),
        }
    }
}

#[derive(Debug)]
pub enum PlayerError {
    NotFound,
    InvalidData,
}
