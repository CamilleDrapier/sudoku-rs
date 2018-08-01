use models::cells::*;
use Sudoku;

pub struct Grid {
    pub number_found: u8,
    pub value: [[Cell; 9]; 9]
}

pub fn build_grid(input: Sudoku) -> Grid {
    let mut result: Grid = Grid{
        number_found: 0,
        value: [[build_cell(); 9]; 9]
    };
    for (i, line) in input.iter().enumerate() {
        for j in 0..line.len() {
            init_grid_cell(&mut result, input, i, j);
        }
    }
    result
}

impl Grid {
    pub fn set_value(&mut self, i: usize, j: usize, value: Option<u8>) {
        self.value[i][j].value = value;
        self.number_found = self.number_found + 1
    }

    pub fn get_cell(&mut self, i: usize, j: usize) -> &Cell {
        &self.value[i][j]
    }
}

fn init_grid_cell(grid: &mut Grid, input: Sudoku, i: usize, j: usize) {

    let cell = &mut grid.value[i][j];
    cell.value = input[i][j];
    if cell.value.is_some() {
        grid.number_found = grid.number_found + 1;
    }
    cell.h_position = Some(i);
    cell.v_position = Some(j);


    for (x, val) in input[i].iter().enumerate() {
        cell.line[x] = *val;
    }

    for (x, line) in input.iter().enumerate() {
        cell.column[x] = line[j];
    }

    let i_region = (i / 3) * 3;
    let j_region = (j / 3) * 3;
    let mut r = 0;
    for k in i_region..(i_region + 3) {
        for l in j_region..(j_region + 3) {
            cell.region[r] = input[k][l];
            r = r + 1;
        }
    }
}

pub fn to_sudoku(grid: &Grid) -> Sudoku {
    let mut sudoku= [[None; 9]; 9];
    for (i, line) in grid.value.iter().enumerate() {
        for (j, cell) in line.iter().enumerate() {
            sudoku[i][j] = cell.value;
        }
    }
    sudoku
}