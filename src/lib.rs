pub mod models;

pub type Sudoku = [[Option<u8>; 9]; 9];

pub fn solve(sud: Sudoku) -> Sudoku {
    models::grids::Grid::from(sud).solve()
}