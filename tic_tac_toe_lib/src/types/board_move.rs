use crate::types::board_dimension::parse as parse_board_dimension;
use crate::types::board_dimension::BoardDimension;
use crate::types::errors::TicTacToeError;
use anyhow::bail;

pub struct BoardMove {
    value: u8,
    board_dimension: BoardDimension,
}

impl BoardMove {
    pub fn get_indices(&self) -> (u8, u8) {
        let row_idx = (self.value - 1) / self.board_dimension.get_value();
        let col_idx = (self.value - 1) % self.board_dimension.get_value();
        (row_idx, col_idx)
    }
}

pub fn parse(board_dimension: &BoardDimension, s: &str) -> Result<BoardMove, TicTacToeError> {
    let str = s.trim();

    let parse_result = str.parse::<u8>();
    if parse_result.is_err() {
        bail!("Move '{}' is not a number", s);
    }

    let val = parse_result.unwrap();

    let min_value = 1;
    if val < min_value {
        bail!("Move can not be less than {}", min_value);
    }

    let max_value = board_dimension.get_value().pow(2);
    if val > max_value {
        bail!("Move can not be greater than {}", max_value);
    }

    Ok(BoardMove {
        value: val,
        board_dimension: board_dimension.clone(),
    })
}

#[test]
fn test_get_indices() {
    let board_dimension = parse_board_dimension("3").unwrap();

    let board_move = parse(&board_dimension, "3").unwrap();
    let (row_idx, col_idx) = board_move.get_indices();
    assert_eq!(row_idx, 0);
    assert_eq!(col_idx, 2);

    let board_move = parse(&board_dimension, "7").unwrap();
    let (row_idx, col_idx) = board_move.get_indices();
    assert_eq!(row_idx, 2);
    assert_eq!(col_idx, 0);

    let board_move = parse(&board_dimension, "9").unwrap();
    let (row_idx, col_idx) = board_move.get_indices();
    assert_eq!(row_idx, 2);
    assert_eq!(col_idx, 2);
}

#[test]
#[should_panic(expected = "Move '' is not a number")]
fn test_parse_empty() {
    let board_dimension = parse_board_dimension("3").unwrap();
    parse(&board_dimension, "").unwrap();
}

#[test]
#[should_panic(expected = "Move ' \t ' is not a number")]
fn test_parse_spaces_only() {
    let board_dimension = parse_board_dimension("3").unwrap();
    parse(&board_dimension, " \t ").unwrap();
}

#[test]
#[should_panic(expected = "Move 'abc' is not a number")]
fn test_parse_not_a_number() {
    let board_dimension = parse_board_dimension("3").unwrap();
    parse(&board_dimension, "abc").unwrap();
}

#[test]
#[should_panic(expected = "Move can not be less than 1")]
fn test_parse_less_than_min() {
    let board_dimension = parse_board_dimension("3").unwrap();
    parse(&board_dimension, "0 ").unwrap();
}

#[test]
#[should_panic(expected = "Move can not be greater than 16")]
fn test_parse_greater_than_max() {
    let board_dimension = parse_board_dimension("4").unwrap();
    parse(&board_dimension, " 100").unwrap();
}

#[test]
fn test_parse_success() {
    let num = 7;
    let board_dimension = parse_board_dimension("3").unwrap();
    let board_move = parse(&board_dimension, num.to_string().as_str()).unwrap();
    assert_eq!(board_move.value, num);
}
