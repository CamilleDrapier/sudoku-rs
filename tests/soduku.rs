extern crate sudoku;
use sudoku::models::grids::Grid;
use sudoku::models::sudokus::Sudoku;

const RESOLVED : Sudoku = [
    [Some(8), Some(9), Some(6), Some(3), Some(4), Some(2), Some(5), Some(1), Some(7)],
    [Some(5), Some(3), Some(4), Some(6), Some(7), Some(1), Some(8), Some(2), Some(9)],
    [Some(2), Some(1), Some(7), Some(9), Some(5), Some(8), Some(3), Some(6), Some(4)],
    [Some(4), Some(2), Some(1), Some(7), Some(8), Some(3), Some(9), Some(5), Some(6)],
    [Some(7), Some(5), Some(8), Some(2), Some(6), Some(9), Some(4), Some(3), Some(1)],
    [Some(9), Some(6), Some(3), Some(5), Some(1), Some(4), Some(2), Some(7), Some(8)],
    [Some(1), Some(4), Some(5), Some(8), Some(3), Some(7), Some(6), Some(9), Some(2)],
    [Some(3), Some(8), Some(9), Some(1), Some(2), Some(6), Some(7), Some(4), Some(5)],
    [Some(6), Some(7), Some(2), Some(4), Some(9), Some(5), Some(1), Some(8), Some(3)],
];

const UNSOLVED : Sudoku = [
    [None, Some(9), Some(6), None, None, None, Some(5), Some(1), None],
    [None, None, None, Some(6), Some(7), Some(1), None, None, None],
    [Some(2), None, Some(7), None, None, None, Some(3), None, Some(4)],
    [Some(4), None, None, Some(7), None, Some(3), None, None, Some(6)],
    [None, None, None, None, None, None, None, None, None],
    [Some(9), None, None, Some(5), None, Some(4), None, None, Some(8)],
    [Some(1), None, Some(5), None, None, None, Some(6), None, Some(2)],
    [None, None, None, Some(1), Some(2), Some(6), None, None, None],
    [None, Some(7), Some(2), None, None, None, Some(1), Some(8), None],
];

const EMPTY : Sudoku = [
    [None, None, None, None, None, None, None, None, None],
    [None, None, None, None, None, None, None, None, None],
    [None, None, None, None, None, None, None, None, None],
    [None, None, None, None, None, None, None, None, None],
    [None, None, None, None, None, None, None, None, None],
    [None, None, None, None, None, None, None, None, None],
    [None, None, None, None, None, None, None, None, None],
    [None, None, None, None, None, None, None, None, None],
    [None, None, None, None, None, None, None, None, None],
];

#[test]
fn it_inits_all_none() {
    let grid = Grid::from([[None; 9]; 9]);
    assert_eq!(grid.sudoku[0][0], None);
    assert_eq!(grid.sudoku[8][8], None);
}

#[test]
fn it_solves() {
    let result = Grid::from(UNSOLVED).solve();
    assert_eq!(RESOLVED, result);
}

#[test]
fn it_returns_a_solved_grid() {
    let result = Grid::from(RESOLVED).solve();
    assert_eq!(RESOLVED, result);
}

#[test]
fn it_gets_values() {
    let result = Grid::from(UNSOLVED);
    assert_eq!(result.get_value(0, 0), None);
    assert_eq!(result.get_value(0, 1), Some(9));
}

#[test]
fn it_finds_potentials_in_an_empty_cell() {
    let result = Grid::from(EMPTY);
    assert_eq!(result.find_potentials(0, 0), vec![Some(1), Some(2), Some(3), Some(4), Some(5), Some(6), Some(7), Some(8), Some(9)]);
}

#[test]
fn it_finds_potentials_in_a_cell() {
    let result = Grid::from(UNSOLVED);
    assert_eq!(result.find_potentials(0, 0), vec![Some(3), Some(8)]);
}

#[test]
fn it_finds_no_potentials_in_a_cell() {
    let result = Grid::from(RESOLVED);
    assert_eq!(result.find_potentials(0, 0), vec![]);
}

#[test]
fn it_checks_impossible() {
    let result = Grid::from(UNSOLVED);
    assert_eq!(result.check_impossible(0, 0, &Some(9)), true);
    assert_eq!(result.check_impossible(0, 0, &Some(3)), false);
    assert_eq!(result.check_impossible(0, 0, &Some(8)), false);
    assert_eq!(result.check_impossible(4, 8, &Some(8)), true);
}