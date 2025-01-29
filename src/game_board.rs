use crate::types::{BoardError, Symbol};

#[derive(Debug, Clone)]
pub struct GameBoard {
    cells: Vec<Vec<Option<Symbol>>>,
    size: usize,
}

impl GameBoard {
    pub fn new(size: usize) -> Self {
        let cells = vec![vec![None; size]; size];
        Self { cells, size }
    }

    pub fn empty_positions(&self) -> Vec<(usize, usize)> {
        let mut positions = Vec::new();
        for i in 0..self.size {
            for j in 0..self.size {
                if self.cells[i][j].is_none() {
                    positions.push((i, j));
                }
            }
        }
        positions
    }

    pub fn has_winning_streak(&self, streak_length: usize) -> Option<(usize, usize)> {
        // Check rows
        for row in 0..self.size {
            for col in 0..=self.size - streak_length {
                if let Some(symbol) = self.cells[row][col] {
                    let mut is_streak = true;
                    for i in 1..streak_length {
                        if self.cells[row][col + i] != Some(symbol) {
                            is_streak = false;
                            break;
                        }
                    }
                    if is_streak {
                        return Some((row, col));
                    }
                }
            }
        }

        // Check columns
        for col in 0..self.size {
            for row in 0..=self.size - streak_length {
                if let Some(symbol) = self.cells[row][col] {
                    let mut is_streak = true;
                    for i in 1..streak_length {
                        if self.cells[row + i][col] != Some(symbol) {
                            is_streak = false;
                            break;
                        }
                    }
                    if is_streak {
                        return Some((row, col));
                    }
                }
            }
        }

        // Check diagonals (top-left to bottom-right)
        for row in 0..=self.size - streak_length {
            for col in 0..=self.size - streak_length {
                if let Some(symbol) = self.cells[row][col] {
                    let mut is_streak = true;
                    for i in 1..streak_length {
                        if self.cells[row + i][col + i] != Some(symbol) {
                            is_streak = false;
                            break;
                        }
                    }
                    if is_streak {
                        return Some((row, col));
                    }
                }
            }
        }

        // Check diagonals (top-right to bottom-left)
        for row in 0..=self.size - streak_length {
            for col in (streak_length - 1)..self.size {
                if let Some(symbol) = self.cells[row][col] {
                    let mut is_streak = true;
                    for i in 1..streak_length {
                        if self.cells[row + i][col - i] != Some(symbol) {
                            is_streak = false;
                            break;
                        }
                    }
                    if is_streak {
                        return Some((row, col));
                    }
                }
            }
        }

        None
    }

    pub fn get_available_positions(&self) -> Vec<(usize, usize)> {
        let mut positions = Vec::new();
        for row in 0..self.size {
            for col in 0..self.size {
                if self.cells[row][col].is_none() {
                    positions.push((row, col));
                }
            }
        }
        positions
    }

    pub fn record_move(
        &mut self,
        position: (usize, usize),
        symbol: Symbol,
    ) -> Result<(), BoardError> {
        let (row, col) = position;

        if row >= self.size || col >= self.size {
            return Err(BoardError::OutOfBounds);
        }

        if self.cells[row][col].is_some() {
            return Err(BoardError::CellOccupied);
        }

        self.cells[row][col] = Some(symbol);
        Ok(())
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn is_full(&self) -> bool {
        self.empty_positions().is_empty()
    }

    pub fn get_cell(&self, position: (usize, usize)) -> Option<Symbol> {
        if !self.is_valid_position(position) {
            None
        } else {
            let (row, col) = position;
            self.cells[row][col]
        }
    }

    fn is_valid_position(&self, pos: (usize, usize)) -> bool {
        let (row, col) = pos;
        row < self.size && col < self.size
    }

    pub fn apply_move(&mut self, pos: (usize, usize), symbol: Symbol) -> Result<(), BoardError> {
        if !self.is_valid_position(pos) {
            return Err(BoardError::OutOfBounds);
        }
        let (row, col) = pos;
        if self.cells[row][col].is_some() {
            return Err(BoardError::CellOccupied);
        }
        self.cells[row][col] = Some(symbol);
        Ok(())
    }

    pub fn undo_move(&mut self, pos: (usize, usize)) -> Result<(), BoardError> {
        if !self.is_valid_position(pos) {
            return Err(BoardError::OutOfBounds);
        }
        let (row, col) = pos;
        self.cells[row][col] = None;
        Ok(())
    }

    pub fn evaluate(&self, ai_symbol: Symbol, player_symbol: Symbol) -> i32 {
        if let Some((row, col)) = self.has_winning_streak(3) {
            if let Some(winner) = self.get_cell((row, col)) {
                if winner == ai_symbol {
                    return 10;
                } else if winner == player_symbol {
                    return -10;
                }
            }
        }
        0
    }

    pub fn get_all_symbols(&self) -> Vec<Symbol> {
        let mut symbols = Vec::new();
        for row in &self.cells {
            for cell in row {
                if let Some(symbol) = cell {
                    if !symbols.contains(symbol) {
                        symbols.push(*symbol);
                    }
                }
            }
        }
        symbols
    }

    pub fn get_winning_positions(&mut self, symbol: Symbol) -> Vec<(usize, usize)> {
        let mut positions = Vec::new();

        // Check each empty position
        for row in 0..self.size {
            for col in 0..self.size {
                if self.cells[row][col].is_none() {
                    // Try the move
                    self.cells[row][col] = Some(symbol);

                    // Check if it's a winning move
                    if self.has_winning_streak(3).is_some() {
                        positions.push((row, col));
                    }

                    // Undo the move
                    self.cells[row][col] = None;
                }
            }
        }

        positions
    }
}
