use Sudoku;

pub struct Grid {
    pub sudoku: Sudoku,
    pub number_found: u8,
}

pub fn build_grid(sudoku: Sudoku) -> Grid {
    let mut number_found = 0;
    for lines in sudoku.iter() {
        for cells in lines.iter() {
            match cells {
                Some(_) => number_found = number_found + 1,
                None => ()
            }
        }
    }
    Grid{
        sudoku,
        number_found
    }
}

impl Grid {
    pub fn set_value(&mut self, i: usize, j: usize, value: Option<u8>) {
        self.sudoku[i][j] = value;
        self.number_found = self.number_found + 1
    }

    pub fn get_value(&self, i: usize, j: usize) -> Option<u8> {
        self.sudoku[i][j]
    }

    pub fn find_potentials(&self, i: usize, j: usize) -> Vec<Option<u8>> {
        match self.get_value(i, j) {
            None => {
                let mut result: Vec<Option<u8>> = vec![Some(1), Some(2), Some(3), Some(4), Some(5), Some(6), Some(7), Some(8), Some(9)];
                for region in [self.line(i), self.column(j), self.region(i, j)].iter() {
                    for value in region.iter() {
                        if result.contains(value) {
                            //TODO use remove_item one day
                            let pos = result.iter().position(|x| *x == *value);
                            result.remove(pos.unwrap());
                        }
                    }
                }
                result
            },
            Some(_) => vec![]
        }
    }

    pub fn line(&self, i: usize) -> [Option<u8>; 9] {
        self.sudoku[i]
    }

    pub fn column(&self, j: usize) -> [Option<u8>; 9] {
        let mut result = [None; 9];
        for (i, val) in self.sudoku.iter().enumerate() {
            result[i] = val[j]
        }
        result
    }

    pub fn region(&self, i: usize, j: usize) -> [Option<u8>; 9] {
        let mut result = [None; 9];
        let i_region = (i / 3) * 3;
        let j_region = (j / 3) * 3;
        let mut r = 0;
        for k in i_region..(i_region + 3) {
            for l in j_region..(j_region + 3) {
                result[r] = self.sudoku[k][l];
                r = r + 1;
            }
        }
        result
    }

    pub fn to_sudoku(&self) -> &Sudoku {
        &self.sudoku
    }
}