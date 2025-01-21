use crate::types::{Symbol, Difficulty, PlayerError};
use crate::ai::AI;
use crate::game_board::GameBoard;

#[derive(Debug, Clone)]
pub struct Player {
    pub name: String,
    symbol: Symbol,
    pub ai: Option<AI>,
}

impl Player {
    pub fn new(name: String, symbol: Symbol, is_human: bool, difficulty: Option<Difficulty>) -> Self {
        let ai = if !is_human {
            Some(AI::new(difficulty.unwrap_or(Difficulty::Medium), symbol.clone()))
        } else {
            None
        };

        Self {
            name,
            symbol,
            ai,
        }
    }

    pub fn load(_id: usize) -> Result<Self, PlayerError> {
        // TODO: Implement loading from storage
        Err(PlayerError::NotFound)
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn symbol(&self) -> Symbol {
        self.symbol
    }

    pub fn is_ai(&self) -> bool {
        self.ai.is_some()
    }

    pub fn get_ai_move(&self, board: &GameBoard) -> (usize, usize) {
        if let Some(ai) = &self.ai {
            ai.decide_move(board)
        } else {
            panic!("Called get_ai_move on a human player")
        }
    }
}
