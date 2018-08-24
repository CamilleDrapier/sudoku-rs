pub mod models;
use models::grids::Grid;
use models::sudokus::Sudoku;

pub fn solve(sud: Sudoku) -> Sudoku {
    Grid::from(sud).solve()
}