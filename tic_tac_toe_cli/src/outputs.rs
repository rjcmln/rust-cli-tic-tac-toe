use crate::game_state::GameState;
use colored::Colorize;

pub fn print_board(state: &GameState) {
    let board_dim = state.board.len();

    println!("---");
    for row_idx in 0..board_dim {
        for col_idx in 0..board_dim {
            let cell_str = format!("{: >3}", &state.board[row_idx][col_idx]);
            if cell_str.trim().parse::<u8>().is_ok() {
                print!("{}", cell_str.bright_black());
            } else {
                print!("{}", cell_str);
            }
        }
        println!();
    }
}

pub fn print_result(result: &str) {
    if result == "Tied" {
        println!("The game is tied");
    } else {
        println!("The winner is {}", result.green());
    }
}
