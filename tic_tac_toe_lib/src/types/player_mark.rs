// NOTE 01: Enum type that can be created only through parse. Dynamic iteration supported too.

use crate::types::errors::TicTacToeError;
use anyhow::bail;
use enum_iterator::IntoEnumIterator;
use std::fmt;

#[derive(Debug, IntoEnumIterator, PartialEq, Clone)]
pub enum PlayerMark {
    X,
    O,
}

const DEFAULT_FIRST_PLAYER_MARK: PlayerMark = PlayerMark::X;

impl fmt::Display for PlayerMark {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub fn get_all_as_vec_str() -> Vec<String> {
    PlayerMark::into_enum_iter()
        .map(|pm| pm.to_string())
        .collect()
}

pub fn get_default() -> PlayerMark {
    DEFAULT_FIRST_PLAYER_MARK
}

pub fn parse(s: &str) -> Result<PlayerMark, TicTacToeError> {
    let str = s.trim();

    if str.len() == 0 {
        return Ok(DEFAULT_FIRST_PLAYER_MARK);
    }

    for pm in PlayerMark::into_enum_iter() {
        if str.to_lowercase() == pm.to_string().to_lowercase() {
            return Ok(pm);
        }
    }

    bail!("Player Mark '{}' is not recognized", s);
}

#[test]
fn test_parse_empty_return_default() {
    assert_eq!(parse("").unwrap(), DEFAULT_FIRST_PLAYER_MARK);
    assert_eq!(parse(" ").unwrap(), DEFAULT_FIRST_PLAYER_MARK);
    assert_eq!(parse("\t").unwrap(), DEFAULT_FIRST_PLAYER_MARK);
}

#[test]
fn test_parse_lower_x_return_x() {
    assert_eq!(parse("x").unwrap(), PlayerMark::X);
}

#[test]
fn test_parse_o_with_spaces_return_o() {
    assert_eq!(parse(" O ").unwrap(), PlayerMark::O);
}

#[test]
#[should_panic(expected = "Player Mark ' abc ' is not recognized")]
fn test_parse_wrong_string() {
    let _pm = parse(" abc ").unwrap();
}

#[test]
fn test_business_rules() {
    assert_eq!(DEFAULT_FIRST_PLAYER_MARK, PlayerMark::X);
}
