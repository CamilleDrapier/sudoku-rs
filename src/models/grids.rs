use models::cells::*;
use Sudoku;

pub struct Grid<'a> {
    sudoku: Sudoku,
    pub number_found: u8,
    pub value: [[Cell<'a>; 9]; 9]
}

pub fn build_grid<'a>(sudoku: Sudoku) -> Grid<'a> {
    let mut grid = Grid{
        sudoku,
        number_found: 0,
        value: [[build_cell(); 9]; 9]
    };
    for i in 0..9 {
        for j in 0..9 {
            grid.init_grid_cell(i, j);
        }
    }
    grid
}

impl<'a> Grid<'a> {
    pub fn set_value(&mut self, i: usize, j: usize, value: Option<u8>) {
        self.sudoku[i][j] = value;
        self.value[i][j].value = value;
        self.number_found = self.number_found + 1
    }

    pub fn get_cell(&mut self, i: usize, j: usize) -> &Cell {
        &self.value[i][j]
    }

    fn init_grid_cell(&mut self, i: usize, j: usize) {

        let cell = &mut self.value[i][j];
        cell.value = self.sudoku[i][j];
        if cell.value.is_some() {
            self.number_found = self.number_found + 1;
        }
        cell.h_position = Some(i);
        cell.v_position = Some(j);


        for (x, val) in self.sudoku[i].iter().enumerate() {
            cell.line[x] = *val;
        }

        for (x, line) in self.sudoku.iter().enumerate() {
            cell.column[x] = line[j];
        }

        let i_region = (i / 3) * 3;
        let j_region = (j / 3) * 3;
        let mut r = 0;
        for k in i_region..(i_region + 3) {
            for l in j_region..(j_region + 3) {
                cell.region[r] = self.sudoku[k][l];
                r = r + 1;
            }
        }
    }

    pub fn to_sudoku(&self) -> &Sudoku {
        &self.sudoku
    }
}