#[derive(Debug, Clone)]
pub struct Player {
    pub symbol: char,
    pub name: String,
    pub win_count: u32,
    pub total_games: u32,
}

impl Player {
    pub fn new(symbol: char, name: String) -> Self {
        Self {
            symbol,
            name,
            win_count: 0,
            total_games: 0,
        }
    }
}
