use crate::game::Game;

pub fn print_stats(game: &Game) {
    println!("╔═══════════════════════════════════════════════════════════╗");
    println!("║                     Game Statistics                       ║");
    println!("╠═══════════╦═══════════╦═══════════╦═══════════╦═══════════╣");
    println!("║  Player   ║   Wins    ║   Games   ║  Win Rate ║  Symbol   ║");
    println!("╠═══════════╬═══════════╬═══════════╬═══════════╬═══════════╣");

    for player in &game.players {
        let win_rate = if player.total_games > 0 {
            (player.win_count as f32 / player.total_games as f32) * 100.0
        } else {
            0.0
        };

        println!(
            "║ {:<9} ║ {:^9} ║ {:^9} ║ {:>7.2}%  ║ {:^8}  ║",
            player.name, player.win_count, player.total_games, win_rate, player.symbol
        );
    }

    println!("╚═══════════╩═══════════╩═══════════╩═══════════╩═══════════╝");
}
pub fn print_board(game: &Game) {
    let board = &game.board_history[game.board_history.len() - 1];
    let size = board.game_state.len();

    println!("Current Board State:");

    // Print top border
    print!("┌");
    for _ in 0..size - 1 {
        print!("───┬");
    }
    println!("───┐");

    // Print rows
    for (i, row) in board.game_state.iter().enumerate() {
        print!("│");
        for &cell in row {
            let display_char = if cell == '_' { ' ' } else { cell };
            print!(" {} │", display_char);
        }
        println!();

        // Print row separator, except for the last row
        if i < size - 1 {
            print!("├");
            for _ in 0..size - 1 {
                print!("───┼");
            }
            println!("───┤");
        }
    }

    // Print bottom border
    print!("└");
    for _ in 0..size - 1 {
        print!("───┴");
    }
    println!("───┘");
}
