use crate::types::player_mark::PlayerMark;
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub struct CellValue(Option<PlayerMark>);

impl fmt::Display for CellValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl CellValue {
    pub fn new(val: Option<PlayerMark>) -> CellValue {
        CellValue(val)
    }

    pub fn get_value(&self) -> &Option<PlayerMark> {
        &self.0
    }

    pub fn set_value(&mut self, val: Option<PlayerMark>) {
        self.0 = val;
    }
}
