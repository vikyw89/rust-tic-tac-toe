use crate::game::Game;
use crate::game_board::GameBoard;
use crate::types::Difficulty;
use std::io::{self, Write};
use std::ops::RangeInclusive;

pub struct UI {
    // Add fields as necessary
}

impl Default for UI {
    fn default() -> Self {
        Self::new()
    }
}

impl UI {
    pub fn new() -> Self {
        Self {
            // Initialize fields as necessary
        }
    }

    pub fn get_game_settings(&self) -> (usize, usize) {
        println!("\nWelcome to Tic Tac Toe!");
        println!("------------------------");

        let board_size = self.get_number_input("Enter board size (default: 3): ", 3, 3..=10);

        let num_players = self.get_number_input("Enter number of players (default: 2): ", 2, 2..=4);

        (board_size, num_players)
    }

    pub fn get_player_type(&self, player_num: usize) -> bool {
        loop {
            print!(
                "Player {} - Enter type (1 for Human, 2 for AI) [default: 1]: ",
                player_num
            );
            io::stdout().flush().unwrap();

            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            let input = input.trim();

            if input.is_empty() {
                return true; // Default to human
            }

            match input.parse::<u32>() {
                Ok(1) => return true,
                Ok(2) => return false,
                _ => println!("Invalid input! Please enter 1 or 2."),
            }
        }
    }

    pub fn get_player_name(&self, player_num: usize, is_human: bool) -> String {
        if !is_human {
            return format!("Computer {}", player_num);
        }

        loop {
            print!(
                "Enter name for Player {} [default: Player {}]: ",
                player_num, player_num
            );
            io::stdout().flush().unwrap();

            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            let input = input.trim();

            if input.is_empty() {
                return format!("Player {}", player_num);
            }

            if !input.is_empty() {
                return input.to_string();
            }
        }
    }

    pub fn get_ai_difficulty(&self) -> Difficulty {
        loop {
            println!("Select AI difficulty:");
            println!("1. Easy");
            println!("2. Medium (default)");
            println!("3. Hard");
            print!("Enter choice [1-3]: ");
            io::stdout().flush().unwrap();

            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            let input = input.trim();

            if input.is_empty() {
                return Difficulty::Medium;
            }

            match input {
                "1" => return Difficulty::Easy,
                "2" => return Difficulty::Medium,
                "3" => return Difficulty::Hard,
                _ => println!("Invalid choice! Please enter a number between 1 and 3."),
            }
        }
    }

    fn get_number_input(
        &self,
        prompt: &str,
        default: usize,
        range: RangeInclusive<usize>,
    ) -> usize {
        loop {
            print!("{}", prompt);
            io::stdout().flush().unwrap();

            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            let input = input.trim();

            if input.is_empty() {
                return default;
            }

            match input.parse::<usize>() {
                Ok(n) if range.contains(&n) => return n,
                _ => println!(
                    "Invalid input! Please enter a number between {} and {}.",
                    range.start(),
                    range.end()
                ),
            }
        }
    }

    pub fn display(&self, game: &Game) {
        self.display_board(game.board());
        if let Some(current) = game.current_player() {
            println!(
                "\nCurrent player: {} ({})",
                current.name(),
                current.symbol()
            );
        } else {
            println!("\nNo current player");
        }
    }

    pub fn display_board(&self, board: &GameBoard) {
        let size = board.size();
        let border = "-".repeat(4 * size + 1);

        // Print column numbers
        print!("   ");
        for col in 0..size {
            print!(" {}  ", col);
        }
        println!();

        println!("{}", border);

        for row in 0..size {
            // Print row number
            print!("{} |", row);

            // Print cells
            for col in 0..size {
                match board.get_cell((row, col)) {
                    Some(symbol) => print!(" {} |", symbol),
                    None => print!("   |"),
                }
            }
            println!();
            println!("{}", border);
        }
    }

    pub fn get_player_move(&self, game: &mut Game) -> (usize, usize) {
        loop {
            print!("Enter your move (row col): ");
            io::stdout().flush().unwrap();

            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();

            if let Some((row, col)) = self.parse_move(&input) {
                if game.is_move_valid((row, col)) {
                    return (row, col);
                }
            }

            println!("Invalid move! Please try again.");
        }
    }

    fn parse_move(&self, input: &str) -> Option<(usize, usize)> {
        let coords: Vec<&str> = input.split_whitespace().collect();
        if coords.len() != 2 {
            return None;
        }

        let row = coords[0].parse::<usize>().ok()?;
        let col = coords[1].parse::<usize>().ok()?;

        Some((row, col))
    }
}
