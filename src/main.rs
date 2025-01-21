mod game;
mod game_board;
mod player;
mod ai;
mod score_board;
mod ui;
mod types;

use game::Game;
use player::Player;
use types::Symbol;
use ui::UI;

fn main() {
    let mut ui = UI::new();
    
    // Get game settings
    let (board_size, num_players) = ui.get_game_settings();
    let mut game = Game::new(board_size, num_players);

    // Create and add players
    for i in 1..=num_players {
        let is_human = ui.get_player_type(i);
        let name = ui.get_player_name(i, is_human);
        
        // Get previous players' symbols to avoid duplicates
        let used_symbols: Vec<Symbol> = game.players()
            .iter()
            .map(|p| p.symbol().clone())
            .collect();

        let difficulty = if !is_human {
            Some(ui.get_ai_difficulty())
        } else {
            None
        };

        let player = Player::new(
            name,
            Symbol::random_unique(&used_symbols).unwrap(),
            is_human,
            difficulty,
        );

        game.add_player(&player).unwrap();
    }

    // Game loop
    while !game.is_over() {
        ui.display(&game);
        
        let position = if let Some(current_player) = game.current_player() {
            if current_player.is_ai() {
                current_player.get_ai_move(game.board())
            } else {
                ui.get_player_move(&mut game)
            }
        } else {
            println!("No current player!");
            break;
        };
        
        if let Err(e) = game.make_move(position) {
            println!("Error: {}", e);
            continue;
        }
    }

    // Display final board and winner
    ui.display(&game);
    if let Some(winner) = game.winner() {
        println!("Winner: {} ({})", winner.name(), winner.symbol());
    } else {
        println!("Game ended in a draw!");
    }
}
