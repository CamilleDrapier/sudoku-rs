use Sudoku;

pub struct Grid {
    pub sudoku: Sudoku,
    pub number_found: u8,
}

impl From<Sudoku> for Grid {
    fn from(sudoku: Sudoku) -> Self {
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
}

impl Grid {
    pub fn set_value(&mut self, i: usize, j: usize, value: Option<u8>) {
        self.sudoku[i][j] = value;
        self.number_found = self.number_found + 1
    }

    pub fn get_value(&self, i: usize, j: usize) -> Option<u8> {
        self.sudoku[i][j]
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

    pub fn check_impossible(&self, i: usize, j: usize, candidate: &Option<u8>) -> bool {
        let mut line_impossible = true;
        for (x, cell) in self.sudoku[i].iter().enumerate() {
            if x != j {
                line_impossible = line_impossible &&
                    (
                        cell.is_some() ||
                            self.line(i).contains(candidate) ||
                            self.column(x).contains(candidate) ||
                            self.region(i, x).contains(candidate)
                    )
            }
        }
        let mut column_impossible = true;
        for (x, line) in self.sudoku.iter().enumerate() {
            if x != i {
                for (y, cell) in line.iter().enumerate() {
                    if y == j {
                        column_impossible = column_impossible &&
                            (
                                cell.is_some() ||
                                    self.line(x).contains(candidate) ||
                                    self.column(y).contains(candidate) ||
                                    self.region(x, y).contains(candidate)
                            )
                    }
                }
            }
        }
        line_impossible || column_impossible
    }

    pub fn solve(&mut self) -> Sudoku {
        let mut prev_number = 0;
        while self.number_found < (9 * 9) && self.number_found > prev_number {
            prev_number = self.number_found;
            for i in 0..9 {
                for j in 0..9 {
                    if self.get_value(i, j).is_none() {
                        let potentials = self.find_potentials(i, j);
                        if potentials.len() == 1 {
                            //println!("Found [{}][{}] -> {:?}", i, j, *potentials.first().unwrap());
                            self.set_value(i, j,*potentials.first().unwrap());
                        }else {
                            for potential in potentials.iter() {
                                if self.get_value(i, j).is_none() && self.check_impossible(i, j, potential) {
                                    //println!("Found Impossible [{}][{}] -> {:?}", i, j, potential.unwrap());
                                    self.set_value(i, j,*potential);
                                }
                            }
                        }
                    }
                }
            }
        }
        self.to_sudoku()
    }

    pub fn to_sudoku(&self) -> Sudoku {
        self.sudoku
    }
}