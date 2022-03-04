use crate::types::board_dimension::{
    get_default as get_default_board_dimension, parse as parse_board_dimension, BoardDimension,
};
use crate::types::errors::TicTacToeError;
use crate::types::player_mark::{
    get_default as get_default_player_mark, parse as parse_player_mark, PlayerMark,
};
use anyhow::bail;

#[derive(Debug, PartialEq)]
pub struct Settings {
    pub first_player: PlayerMark,
    pub board_dimension: BoardDimension,
}

pub fn parse(s: &str) -> Result<Settings, TicTacToeError> {
    let str = s.trim();

    if str.len() == 0 {
        return Ok(Settings {
            first_player: get_default_player_mark(),
            board_dimension: get_default_board_dimension(),
        });
    }

    let parts: Vec<&str> = str.split(",").collect();
    match parts.len() {
        1 => {
            let fp = parse_player_mark(parts[0])?;
            return Ok(Settings {
                first_player: fp,
                board_dimension: get_default_board_dimension(),
            });
        }
        2 => {
            let fp = parse_player_mark(parts[0])?;
            let bd = parse_board_dimension(parts[1])?;
            return Ok(Settings {
                first_player: fp,
                board_dimension: bd,
            });
        }
        _ => bail!("More than 2 settings provided: '{}'", s),
    }
}

#[test]
fn test_parse_empty_return_default() {
    let default_settings = Settings {
        first_player: get_default_player_mark(),
        board_dimension: get_default_board_dimension(),
    };
    assert_eq!(parse("").unwrap(), default_settings);
    assert_eq!(parse(" ").unwrap(), default_settings);
    assert_eq!(parse("\t").unwrap(), default_settings);
}

#[test]
fn test_parse_one_argument_success() {
    assert_eq!(
        parse("x").unwrap(),
        Settings {
            first_player: PlayerMark::X,
            board_dimension: get_default_board_dimension(),
        }
    );
    assert_eq!(
        parse(" O ").unwrap(),
        Settings {
            first_player: PlayerMark::O,
            board_dimension: get_default_board_dimension(),
        }
    );
}

#[test]
#[should_panic(expected = "Player Mark 'abc' is not recognized")]
fn test_parse_one_argument_failure() {
    let _s = parse("abc").unwrap();
}

#[test]
fn test_parse_two_arguments_success() {
    assert_eq!(
        parse(" x , 4 ").unwrap(),
        Settings {
            first_player: PlayerMark::X,
            board_dimension: parse_board_dimension("4").unwrap(),
        }
    );
}

#[test]
#[should_panic(expected = "Player Mark 'abc' is not recognized")]
fn test_parse_two_arguments_wrong_player_mark() {
    let _s = parse("abc, 4").unwrap();
}

#[test]
#[should_panic(expected = "Board Dimension '4m' is not a number")]
fn test_parse_two_arguments_board_dimension_not_a_number() {
    let _s = parse("x,4m").unwrap();
}

#[test]
#[should_panic(expected = "Board Dimension can not be greater than ")]
fn test_parse_two_arguments_board_dimension_greater_than_max() {
    let _s = parse("x, 44").unwrap();
}

#[test]
#[should_panic(expected = "More than 2 settings provided")]
fn test_parse_three_arguments() {
    let _s = parse("X,4,O").unwrap();
}
