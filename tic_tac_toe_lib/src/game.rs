use crate::game_state::{deserialize_game_state, GameState};
use crate::settings::parse;
use crate::state_calculation::calculate_result;
use crate::types::board::Board;
use crate::types::board_move::parse as parse_board_move;
use crate::types::errors::TicTacToeError;
use crate::types::player_mark::PlayerMark;
use anyhow::bail;

#[derive(Debug)]
pub struct Game {
    board: Board,
    next_on_move: PlayerMark,
    finished: bool,
    result: Option<String>,
}

impl Game {
    pub fn new(settings_str: &str) -> Result<Game, TicTacToeError> {
        let settings = parse(settings_str)?;

        Ok(Game {
            board: Board::new(settings.board_dimension),
            next_on_move: settings.first_player,
            finished: false,
            result: None,
        })
    }

    pub fn play_move(&mut self, move_str: &str) -> Result<(), TicTacToeError> {
        // check if game is already finished
        if self.finished {
            bail!("Game is finished, you can not play a move");
        }

        // parse move - is str a number, is between 1 and 9/16/25
        let board_move = parse_board_move(&self.board.dimension, move_str)?;
        let (row_idx, col_idx) = board_move.get_indices();

        // check if cell is filled
        let cell = &mut self.board.cells[row_idx as usize][col_idx as usize];
        match cell.get_value() {
            Some(pm) => bail!(
                "Cell {} is already filled with '{}'",
                move_str,
                pm.to_string()
            ),
            None => {}
        };

        // update cell
        cell.set_value(Some(self.next_on_move.clone()));

        // toggle next on move
        self.next_on_move = if self.next_on_move == PlayerMark::X {
            PlayerMark::O
        } else {
            PlayerMark::X
        };

        // calculate winner, all filled -> finished, result
        let (finished, result) = calculate_result(&self.board);
        self.finished = finished;
        self.result = result;

        Ok(())
    }

    pub fn get_state(&self) -> String {
        let board_dim = self.board.dimension.get_value();

        let mut bd: Vec<Vec<String>> = vec![];
        for row_idx in 0..board_dim {
            let mut row: Vec<String> = vec![];
            for col_idx in 0..board_dim {
                let cell_value = self.board.cells[row_idx as usize][col_idx as usize]
                    .get_value()
                    .clone();
                let str = match cell_value {
                    None => (row_idx * board_dim + col_idx + 1).to_string(),
                    Some(pm) => pm.to_string(),
                };
                row.push(str);
            }
            bd.push(row);
        }

        let res: String = match &self.result {
            None => String::from(""),
            Some(res) => res.to_string(),
        };

        let bs = GameState {
            board: bd,
            next_on_move: self.next_on_move.to_string(),
            finished: self.finished,
            result: res,
        };

        serde_json::to_string(&bs).unwrap()
    }
}

#[test]
fn test_new_state() {
    let g = Game::new(" o , 3 ").unwrap();

    let s = "{\"board\":[[\"1\",\"2\",\"3\"],[\"4\",\"5\",\"6\"],[\"7\",\"8\",\"9\"]],\"next_on_move\":\"O\",\"finished\":false,\"result\":\"\"}";
    let state_expected = deserialize_game_state(s);

    let state_actual = deserialize_game_state(&g.get_state());

    assert_eq!(state_expected, state_actual);
}

#[test]
fn test_play_moves_state() {
    let mut g = Game::new(" x , 3 ").unwrap();
    g.play_move("1").unwrap();
    g.play_move("5").unwrap();
    g.play_move("9").unwrap();

    let s = "{\"board\":[[\"X\",\"2\",\"3\"],[\"4\",\"O\",\"6\"],[\"7\",\"8\",\"X\"]],\"next_on_move\":\"O\",\"finished\":false,\"result\":\"\"}";
    let state_expected = deserialize_game_state(s);

    let state_actual = deserialize_game_state(&g.get_state());

    assert_eq!(state_expected, state_actual);
}

#[test]
fn test_restart_state() {
    let mut g = Game::new(" o , 3 ").unwrap();
    g.play_move("1").unwrap();
    g.play_move("5").unwrap();
    g.play_move("9").unwrap();

    g = Game::new(" o , 3 ").unwrap();

    let s = "{\"board\":[[\"1\",\"2\",\"3\"],[\"4\",\"5\",\"6\"],[\"7\",\"8\",\"9\"]],\"next_on_move\":\"O\",\"finished\":false,\"result\":\"\"}";
    let state_expected = deserialize_game_state(s);

    let state_actual = deserialize_game_state(&g.get_state());

    assert_eq!(state_expected, state_actual);
}

#[test]
fn test_detect_finished_winner_x() {
    let mut g = Game::new(" x , 3 ").unwrap();
    // X wins on row 0
    g.play_move("1").unwrap();
    g.play_move("4").unwrap();
    g.play_move("2").unwrap();
    g.play_move("5").unwrap();
    g.play_move("3").unwrap();

    let state_expected = deserialize_game_state(&g.get_state());
    assert_eq!(state_expected.finished, true);
    assert_eq!(state_expected.result, "X");
}

#[test]
fn test_detect_finished_winner_o() {
    let mut g = Game::new(" x , 3 ").unwrap();
    // O wins on right diagonal
    g.play_move("1").unwrap();
    g.play_move("3").unwrap();
    g.play_move("2").unwrap();
    g.play_move("5").unwrap();
    g.play_move("4").unwrap();
    g.play_move("7").unwrap();

    let state_expected = deserialize_game_state(&g.get_state());
    assert_eq!(state_expected.finished, true);
    assert_eq!(state_expected.result, "O");
}

#[test]
fn test_detect_finished_tied() {
    let mut g = Game::new("O,3").unwrap();
    g.play_move("5").unwrap();
    g.play_move("3").unwrap();
    g.play_move("7").unwrap();
    g.play_move("1").unwrap();
    g.play_move("2").unwrap();
    g.play_move("8").unwrap();
    g.play_move("4").unwrap();
    g.play_move("6").unwrap();
    g.play_move("9").unwrap();

    let state_expected = deserialize_game_state(&g.get_state());
    assert_eq!(state_expected.finished, true);
    assert_eq!(state_expected.result, "Tied");
}

#[test]
#[should_panic(expected = "Game is finished, you can not play a move")]
fn test_play_move_after_finished_winner_x() {
    let mut g = Game::new(" x , 3 ").unwrap();
    g.play_move("1").unwrap();
    g.play_move("4").unwrap();
    g.play_move("2").unwrap();
    g.play_move("5").unwrap();
    g.play_move("3").unwrap();

    g.play_move("6").unwrap();
}

#[test]
#[should_panic(expected = "Game is finished, you can not play a move")]
fn test_play_move_after_finished_winner_o() {
    let mut g = Game::new(" x , 4 ").unwrap();
    g.play_move("2").unwrap();
    g.play_move("1").unwrap();
    g.play_move("7").unwrap();
    g.play_move("6").unwrap();
    g.play_move("12").unwrap();
    g.play_move("11").unwrap();
    g.play_move("13").unwrap();
    g.play_move("16").unwrap();

    g.play_move("5").unwrap();
}

#[test]
#[should_panic(expected = "Game is finished, you can not play a move")]
fn test_play_move_after_finished_tied() {
    let mut g = Game::new("O,3").unwrap();
    g.play_move("5").unwrap();
    g.play_move("3").unwrap();
    g.play_move("7").unwrap();
    g.play_move("1").unwrap();
    g.play_move("2").unwrap();
    g.play_move("8").unwrap();
    g.play_move("4").unwrap();
    g.play_move("6").unwrap();
    g.play_move("9").unwrap();

    g.play_move("1").unwrap();
}
