use crate::game::Game;
use crate::player::Player;
use crate::types::{Difficulty, Symbol};
use std::collections::HashMap;

#[derive(Debug, Default)]
struct GameStats {
    wins: HashMap<String, usize>,
    total_games: usize,
    draws: usize,
}

impl GameStats {
    fn new() -> Self {
        Self {
            wins: HashMap::new(),
            total_games: 0,
            draws: 0,
        }
    }

    fn record_win(&mut self, player_name: &str) {
        *self.wins.entry(player_name.to_string()).or_insert(0) += 1;
        self.total_games += 1;
    }

    fn record_draw(&mut self) {
        self.draws += 1;
        self.total_games += 1;
    }

    fn print_summary(&self) {
        println!("\nGame Statistics:");
        println!("Total Games: {}", self.total_games);
        println!(
            "Total Draws: {} ({:.1}%)",
            self.draws,
            (self.draws as f64 / self.total_games as f64) * 100.0
        );
        println!("\nWins by Player:");
        let mut wins: Vec<_> = self.wins.iter().collect();
        wins.sort_by(|a, b| b.1.cmp(a.1));
        for (player, wins) in wins {
            let win_rate = (*wins as f64 / self.total_games as f64) * 100.0;
            println!("{}: {} ({:.1}%)", player, wins, win_rate);
        }
    }
}

#[test]
fn test_three_player_ai_performance() {
    let num_games = 100;
    let board_size = 5;
    let mut stats = GameStats::new();

    // Test different AI combinations
    let difficulties = [
        (Difficulty::Hard, Difficulty::Medium, Difficulty::Easy),
        (Difficulty::Hard, Difficulty::Hard, Difficulty::Hard),
        (Difficulty::Medium, Difficulty::Medium, Difficulty::Medium),
        (Difficulty::Easy, Difficulty::Easy, Difficulty::Easy),
    ];

    for (ai1_diff, ai2_diff, ai3_diff) in difficulties.iter() {
        println!("\nTesting AI Combination:");
        println!("AI 1: {:?}", ai1_diff);
        println!("AI 2: {:?}", ai2_diff);
        println!("AI 3: {:?}", ai3_diff);

        let mut combination_stats = GameStats::new();

        for _ in 0..num_games {
            let mut game = Game::new(board_size, 3);

            // Create players with different AI difficulties
            let player1 = Player::new(
                format!("AI1_{:?}", ai1_diff),
                Symbol::random_unique(&[]).unwrap(),
                false,
                Some(*ai1_diff),
            );
            let player2 = Player::new(
                format!("AI2_{:?}", ai2_diff),
                Symbol::random_unique(&[player1.symbol().clone()]).unwrap(),
                false,
                Some(*ai2_diff),
            );
            let player3 = Player::new(
                format!("AI3_{:?}", ai3_diff),
                Symbol::random_unique(&[player1.symbol().clone(), player2.symbol().clone()])
                    .unwrap(),
                false,
                Some(*ai3_diff),
            );

            game.add_player(&player1).unwrap();
            game.add_player(&player2).unwrap();
            game.add_player(&player3).unwrap();

            // Play the game
            game.randomize_turn();
            while !game.is_over() {
                if let Some(current_player) = game.current_player() {
                    if let Some(ref ai) = current_player.ai {
                        let mv = ai.decide_move(game.board());
                        if let Err(e) = game.make_move(mv) {
                            println!("Error making move: {}", e);
                            break;
                        }
                    }
                } else {
                    break;
                }
            }

            // Record results
            if let Some(winner) = game.winner() {
                combination_stats.record_win(&winner.name);
                stats.record_win(&winner.name);
            } else {
                combination_stats.record_draw();
                stats.record_draw();
            }
        }

        // Print statistics for this combination
        combination_stats.print_summary();
    }

    // Print overall statistics
    println!("\nOverall Statistics:");
    stats.print_summary();
}
