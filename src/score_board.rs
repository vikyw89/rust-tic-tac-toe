use crate::player::Player;
use crate::types::GameResult;
use std::collections::HashMap;

#[derive(Debug, Default)]
pub struct PlayerStats {
    wins: u32,
    losses: u32,
    draws: u32,
}

impl PlayerStats {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_result(&mut self, result: GameResult) {
        match result {
            GameResult::Win => self.wins += 1,
            GameResult::Loss => self.losses += 1,
            GameResult::Draw => self.draws += 1,
        }
    }

    pub fn win_rate(&self) -> f32 {
        let total = self.total_games();
        if total == 0 {
            0.0
        } else {
            self.wins as f32 / total as f32
        }
    }

    pub fn total_games(&self) -> u32 {
        self.wins + self.losses + self.draws
    }
}

#[derive(Debug, Default)]
pub struct ScoreBoard {
    stats: HashMap<String, PlayerStats>,
}

impl ScoreBoard {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn record_result(&mut self, player: &Player, result: GameResult) {
        self.stats
            .entry(player.name().to_string())
            .or_default()
            .add_result(result);
    }

    pub fn get_stats(&self, player: &Player) -> Option<&PlayerStats> {
        self.stats.get(player.name())
    }
}
