use crate::types::errors::TicTacToeError;
use anyhow::bail;

#[derive(Debug, PartialEq, Clone)]
pub struct BoardDimension(u8);

const MIN_VALUE: u8 = 3;
const MAX_VALUE: u8 = 5;
const DEFAULT_VALUE: u8 = 3;

impl BoardDimension {
    pub fn get_value(&self) -> u8 {
        self.0
    }
}

pub fn get_all_as_vec_str() -> Vec<String> {
    let mut vec_str: Vec<String> = vec![];

    for i in MIN_VALUE..=MAX_VALUE {
        vec_str.push(i.to_string());
    }

    vec_str.to_vec()
}

pub fn get_default() -> BoardDimension {
    BoardDimension(DEFAULT_VALUE)
}

pub fn parse(s: &str) -> Result<BoardDimension, TicTacToeError> {
    let str = s.trim();

    if str.len() == 0 {
        return Ok(BoardDimension(DEFAULT_VALUE));
    }

    let parse_result = str.parse::<u8>();
    if parse_result.is_err() {
        bail!("Board Dimension '{}' is not a number", s);
    }

    let val = parse_result.unwrap();
    if val < MIN_VALUE {
        bail!("Board Dimension can not be less than {}", MIN_VALUE);
    }
    if val > MAX_VALUE {
        bail!("Board Dimension can not be greater than {}", MAX_VALUE);
    }

    Ok(BoardDimension(val))
}

#[test]
fn test_parse_empty_return_default() {
    assert_eq!(parse("").unwrap().get_value(), DEFAULT_VALUE);
    assert_eq!(parse(" ").unwrap().get_value(), DEFAULT_VALUE);
    assert_eq!(parse("\t").unwrap().get_value(), DEFAULT_VALUE);
}

#[test]
#[should_panic(expected = "Board Dimension ' a1 ' is not a number")]
fn test_parse_not_a_number() {
    let _bd = parse(" a1 ").unwrap();
}

#[test]
#[should_panic(expected = "Board Dimension can not be less than ")]
fn test_parse_less_than_min() {
    let _bd = parse((MIN_VALUE - 1).to_string().as_str()).unwrap();
}

#[test]
#[should_panic(expected = "Board Dimension can not be greater than ")]
fn test_parse_greater_than_max() {
    let _bd = parse((MAX_VALUE + 1).to_string().as_str()).unwrap();
}

#[test]
fn test_parse_convert_min_value() {
    let bd = parse(MIN_VALUE.to_string().as_str()).unwrap().get_value();
    assert_eq!(bd, MIN_VALUE);
}

#[test]
fn test_parse_convert_max_value() {
    let bd = parse(MAX_VALUE.to_string().as_str()).unwrap().get_value();
    assert_eq!(bd, MAX_VALUE);
}

// TODO: maybe remove, it should be tested through settings_rules.test_get_game_settings_rules
#[test]
fn test_business_rules() {
    assert_eq!(MIN_VALUE, 3);
    assert_eq!(MAX_VALUE, 5);
    assert_eq!(DEFAULT_VALUE, 3);
}
