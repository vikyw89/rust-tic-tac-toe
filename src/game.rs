use crate::board::Board;
use crate::player::Player;
use rand::Rng;

pub struct Game {
    pub board_history: Vec<Board>,
    pub current_board: Board,
    pub turn_history: Vec<Player>,
    pub players: Vec<Player>,
}

impl Game {
    pub fn new(board_size: u32) -> Self {
        let board = Board::new(board_size);
        Self {
            board_history: vec![board.clone()],
            current_board: board.clone(),
            turn_history: Vec::new(),
            players: Vec::new(),
        }
    }

    pub fn add_player(&mut self, name: String, symbol: char) {
        self.players.push(Player::new(symbol, name));
    }

    pub fn randomize_turn(&mut self) {
        // randomize the turn, players vec
        let mut new_player_order: Vec<Player> = vec![];

        while self.players.len() > 0 {
            let mut rng = rand::thread_rng();
            let random_index = rng.gen_range(0..self.players.len());
            let player = self.players[random_index].clone();
            self.players.remove(random_index);
            new_player_order.push(player);
        }

        self.players = new_player_order;
    }

    pub fn get_next_player(&self) -> Option<&Player> {
        // if no players, return None
        if self.players.len() == 0 {
            return None;
        }

        // if no available moves, return None
        if self.current_board.get_empty_coordinates().len() == 0 {
            return None;
        }

        // return next player, using board history as index
        let current_player_index = self.board_history.len() % self.players.len();
        let next_player_index = (current_player_index + 1) % self.players.len();

        Some(&self.players[next_player_index])
    }
}
