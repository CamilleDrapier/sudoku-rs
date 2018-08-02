use models::grids::*;

pub struct Cell<'a> {
    pub grid: Option<&'a Grid<'a>>,
    pub value: Option<u8>,
    pub h_position: Option<usize>,
    pub v_position: Option<usize>,
    pub line: [Option<u8>; 9],
    pub column: [Option<u8>; 9],
    pub region: [Option<u8>; 9]
}

pub fn build_cell<'a>() -> Cell<'a> {
    Cell {
        grid: None,
        value: None,
        h_position: None,
        v_position: None,
        line: [None; 9],
        column: [None; 9],
        region: [None; 9]
    }
}

impl<'a> Cell<'a> {
    pub fn find_potentials(&self) -> Vec<Option<u8>> {
        let mut result: Vec<Option<u8>> = vec![Some(1), Some(2), Some(3), Some(4), Some(5), Some(6), Some(7), Some(8), Some(9)];
        if !self.value.is_some() {
            for region in [self.line, self.column, self.region].iter() {
                for value in region.iter() {
                    if result.contains(value) {
                        //TODO use remove_item one day
                        let pos = result.iter().position(|x| *x == *value);
                        result.remove(pos.unwrap());
                    }
                }
            }
        }
        result
    }
}
impl<'a> Copy for Cell<'a> {}

impl<'a> Clone for Cell<'a> {
    fn clone(&self) -> Self {
        Cell {
            grid: self.grid,
            value: self.value,
            h_position: self.h_position,
            v_position: self.v_position,
            line: self.line,
            column: self.column,
            region: self.region,
        }
    }
}
