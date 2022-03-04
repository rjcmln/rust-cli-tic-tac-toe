use crate::types::board_dimension::BoardDimension;
use crate::types::cell_value::CellValue;

#[derive(Debug)]
pub struct Board {
    pub dimension: BoardDimension,
    pub cells: Vec<Vec<CellValue>>,
}

impl Board {
    pub fn new(dimension: BoardDimension) -> Board {
        let vec_size = dimension.get_value() as usize;

        Board {
            dimension,
            cells: vec![vec![CellValue::new(None); vec_size]; vec_size],
        }
    }
}
