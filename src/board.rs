#[derive(Clone)]
pub struct Board {
    pub game_state: Vec<Vec<char>>,
}

impl Board {
    pub fn new(board_size: u32) -> Self {
        let mut game_state: Vec<Vec<char>> = Vec::new();

        for _i in 0..board_size {
            game_state.push(vec!['_'; board_size as usize]);
        }

        Self { game_state }
    }

    pub fn receive_marking(&mut self, x: u32, y: u32, symbol: char) {
        self.game_state[y as usize][x as usize] = symbol;
    }

    pub fn get_empty_coordinates(&self) -> Vec<(u32, u32)> {
        let mut empty_coordinates: Vec<(u32, u32)> = Vec::new();
        for i in 0..self.game_state.len() {
            for j in 0..self.game_state.len() {
                if self.game_state[i][j] == '_' {
                    empty_coordinates.push((i as u32, j as u32));
                }
            }
        }
        empty_coordinates
    }

    pub fn get_winning_row(&self) -> Option<char> {
        for y in 0..self.game_state.len() {
            let mut symbol_to_check = '_';
            for x in 0..self.game_state[0].len() {
                if self.game_state[y][x] == '_' {
                    break;
                }

                if x == 0 {
                    symbol_to_check = self.game_state[y][x];
                }

                if self.game_state[y][x] != symbol_to_check {
                    break;
                }

                if x == self.game_state[y].len() - 1 {
                    return Some(symbol_to_check);
                }
            }
        }
        None
    }
}
