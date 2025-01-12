pub mod board;
pub mod game;
pub mod player;
pub mod ui;

fn main() {
    // prepare game
    let mut game = game::Game::new(10);

    // add players
    game.add_player("player1".to_string(), 'X');
    game.add_player("player2".to_string(), 'O');

    ui::print_stats(&game);

    // randomize turn
    game.randomize_turn();

    ui::print_stats(&game);
    // ui::print_board(&game);
}
