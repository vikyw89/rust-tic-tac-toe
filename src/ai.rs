use crate::game_board::GameBoard;
use crate::types::{Difficulty, Symbol};
use rand::seq::SliceRandom;

#[derive(Debug, Clone)]
pub struct AI {
    difficulty: Difficulty,
    symbol: Symbol,
}

impl AI {
    pub fn new(difficulty: Difficulty, symbol: Symbol) -> Self {
        Self { difficulty, symbol }
    }

    pub fn decide_move(&self, board: &GameBoard) -> (usize, usize) {
        match self.difficulty {
            Difficulty::Easy => self.random_move(board),
            Difficulty::Medium => {
                if rand::random::<f32>() < 0.7 {
                    self.smart_move(board)
                } else {
                    self.random_move(board)
                }
            }
            Difficulty::Hard => self.smart_move(board),
        }
    }

    fn random_move(&self, board: &GameBoard) -> (usize, usize) {
        let empty_positions = board.empty_positions();
        *empty_positions.choose(&mut rand::thread_rng()).unwrap()
    }

    fn smart_move(&self, board: &GameBoard) -> (usize, usize) {
        let empty_positions = board.empty_positions();

        // If center is empty, take it (good strategy in any case)
        let center = board.size() / 2;
        if empty_positions.contains(&(center, center)) {
            return (center, center);
        }

        // Check for winning moves
        for pos in empty_positions.iter() {
            let mut board_clone = board.clone();
            if board_clone.apply_move(*pos, self.symbol.clone()).is_ok() {
                if board_clone.has_winning_streak(3).is_some() {
                    return *pos;
                }
            }
        }

        // Check for blocking opponent's winning moves
        for pos in empty_positions.iter() {
            let mut board_clone = board.clone();
            for symbol in board_clone.get_all_symbols() {
                if symbol != self.symbol {
                    if board_clone.apply_move(*pos, symbol.clone()).is_ok() {
                        if board_clone.has_winning_streak(3).is_some() {
                            return *pos;
                        }
                    }
                    board_clone.undo_move(*pos);
                }
            }
        }

        // Try to create a fork (two ways to win)
        for pos in empty_positions.iter() {
            let mut board_clone = board.clone();
            if board_clone.apply_move(*pos, self.symbol.clone()).is_ok() {
                let mut winning_paths = 0;
                for next_pos in board_clone.empty_positions() {
                    let mut next_board = board_clone.clone();
                    if next_board.apply_move(next_pos, self.symbol.clone()).is_ok() {
                        if next_board.has_winning_streak(3).is_some() {
                            winning_paths += 1;
                        }
                    }
                }
                if winning_paths >= 2 {
                    return *pos;
                }
            }
        }

        // If no strategic move is found, take a corner if available
        let corners = [
            (0, 0),
            (0, board.size() - 1),
            (board.size() - 1, 0),
            (board.size() - 1, board.size() - 1),
        ];
        for corner in corners.iter() {
            if empty_positions.contains(corner) {
                return *corner;
            }
        }

        // Otherwise, take any available position
        self.random_move(board)
    }

    pub fn adjust_difficulty(&mut self, difficulty: Difficulty) {
        self.difficulty = difficulty;
    }
}
