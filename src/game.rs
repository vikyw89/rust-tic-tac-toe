use crate::game_board::GameBoard;
use crate::player::Player;
use crate::types::{GameError, Symbol};
use rand::Rng;

pub struct Game {
    board: GameBoard,
    players: Vec<Player>,
    current_player_idx: usize,
    max_players: usize,
}

impl Game {
    pub fn new(board_size: usize, max_players: usize) -> Self {
        Self {
            board: GameBoard::new(board_size),
            players: Vec::new(),
            current_player_idx: 0,
            max_players,
        }
    }

    pub fn add_player(&mut self, player: &Player) -> Result<(), GameError> {
        if self.players.len() >= self.max_players {
            return Err(GameError::MaxPlayersReached);
        }
        self.players.push(player.clone());
        Ok(())
    }

    pub fn current_player(&self) -> Option<&Player> {
        if self.players.is_empty() {
            None
        } else {
            Some(&self.players[self.current_player_idx])
        }
    }

    pub fn is_over(&self) -> bool {
        self.winner().is_some() || self.board.is_full()
    }

    pub fn winner(&self) -> Option<&Player> {
        if let Some((row, col)) = self.board.has_winning_streak(3) {
            if let Some(symbol) = self.board.get_cell((row, col)) {
                return self.players.iter().find(|p| p.symbol() == symbol);
            }
        }
        None
    }

    pub fn randomize_turn(&mut self) {
        if !self.players.is_empty() {
            self.current_player_idx = rand::thread_rng().gen_range(0..self.players.len());
        }
    }

    pub fn prepare_next_round(&mut self) {
        self.board = GameBoard::new(self.board.size());
        self.randomize_turn();
    }

    pub fn board(&self) -> &GameBoard {
        &self.board
    }

    pub fn make_move(&mut self, pos: (usize, usize)) -> Result<(), GameError> {
        let current_player = self.current_player().ok_or(GameError::InvalidMove)?;
        self.board.apply_move(pos, current_player.symbol().clone())?;
        self.current_player_idx = (self.current_player_idx + 1) % self.players.len();
        Ok(())
    }

    pub fn is_move_valid(&self, pos: (usize, usize)) -> bool {
        if let Some(current_player) = self.current_player() {
            self.board.get_cell(pos).is_none() && pos.0 < self.board.size() && pos.1 < self.board.size()
        } else {
            false
        }
    }

    pub fn players(&self) -> &Vec<Player> {
        &self.players
    }
}
