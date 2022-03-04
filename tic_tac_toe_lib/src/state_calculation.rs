// NOTE 06: Calculation function that serves as a helper to the Game module.

use crate::types::board::Board;
use crate::types::player_mark::PlayerMark;

pub fn calculate_result(board: &Board) -> (bool, Option<String>) {
    let board_dimension = board.dimension.get_value();

    // Full row
    for row_idx in 0..board_dimension {
        let mut all_in_row_x = true;
        let mut all_in_row_o = true;
        for col_idx in 0..board_dimension {
            let cell_value = board.cells[row_idx as usize][col_idx as usize]
                .get_value()
                .clone();
            if cell_value != Some(PlayerMark::X) {
                all_in_row_x = false;
            }
            if cell_value != Some(PlayerMark::O) {
                all_in_row_o = false;
            }
        }
        if all_in_row_x {
            return (true, Some(PlayerMark::X.to_string()));
        }
        if all_in_row_o {
            return (true, Some(PlayerMark::O.to_string()));
        }
    }

    // Full column
    for col_idx in 0..board_dimension {
        let mut all_in_col_x = true;
        let mut all_in_col_o = true;
        for row_idx in 0..board_dimension {
            let cell_value = board.cells[row_idx as usize][col_idx as usize]
                .get_value()
                .clone();
            if cell_value != Some(PlayerMark::X) {
                all_in_col_x = false;
            }
            if cell_value != Some(PlayerMark::O) {
                all_in_col_o = false;
            }
        }
        if all_in_col_x {
            return (true, Some(PlayerMark::X.to_string()));
        }
        if all_in_col_o {
            return (true, Some(PlayerMark::O.to_string()));
        }
    }

    // Diagonals
    let mut all_in_left_diagonal_x = true;
    let mut all_in_right_diagonal_x = true;
    let mut all_in_left_diagonal_o = true;
    let mut all_in_right_diagonal_o = true;
    for i in 0..board_dimension {
        let cell_value_left_diagonal = board.cells[i as usize][i as usize].get_value().clone();
        let cell_value_right_diagonal = board.cells[i as usize][(board_dimension - 1 - i) as usize]
            .get_value()
            .clone();

        if cell_value_left_diagonal != Some(PlayerMark::X) {
            all_in_left_diagonal_x = false;
        }
        if cell_value_right_diagonal != Some(PlayerMark::X) {
            all_in_right_diagonal_x = false;
        }
        if cell_value_left_diagonal != Some(PlayerMark::O) {
            all_in_left_diagonal_o = false;
        }
        if cell_value_right_diagonal != Some(PlayerMark::O) {
            all_in_right_diagonal_o = false;
        }
    }
    if all_in_left_diagonal_x {
        return (true, Some(PlayerMark::X.to_string()));
    }
    if all_in_right_diagonal_x {
        return (true, Some(PlayerMark::X.to_string()));
    }
    if all_in_left_diagonal_o {
        return (true, Some(PlayerMark::O.to_string()));
    }
    if all_in_right_diagonal_o {
        return (true, Some(PlayerMark::O.to_string()));
    }

    // All cells are filled
    let mut all_filled = true;
    'outer: for row_idx in 0..board_dimension {
        for col_idx in 0..board_dimension {
            let cell_value = board.cells[row_idx as usize][col_idx as usize]
                .get_value()
                .clone();
            if cell_value == None {
                all_filled = false;
                break 'outer;
            }
        }
    }
    if all_filled {
        return (true, Some(String::from("Tied")));
    }

    // Not finished
    (false, None)
}

// NOTE: Tests are integrated in game module tests (test_detect_finished)
//       Otherwise board field in Game would have to become public
