extern crate sudoku;
use sudoku::Sudoku;
use sudoku::models::*;

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
    let grid = sudoku::models::grids::build_grid([[None; 9]; 9]);
    assert_eq!(grid.sudoku[0][0], None);
    assert_eq!(grid.sudoku[8][8], None);
}

#[test]
fn it_solves() {
    let result = sudoku::solve(UNSOLVED);
    assert_eq!(RESOLVED, result);
}

#[test]
fn it_returns_a_solved_grid() {
    let result = sudoku::solve(RESOLVED);
    assert_eq!(RESOLVED, result);
}

#[test]
fn it_gets_values() {
    let result = grids::build_grid(UNSOLVED);
    assert_eq!(result.get_value(0, 0), None);
    assert_eq!(result.get_value(0, 1), Some(9));
}

#[test]
fn it_finds_potentials_in_an_empty_cell() {
    let result = grids::build_grid(EMPTY);
    assert_eq!(result.find_potentials(0, 0), vec![Some(1), Some(2), Some(3), Some(4), Some(5), Some(6), Some(7), Some(8), Some(9)]);
}

#[test]
fn it_finds_potentials_in_a_cell() {
    let result = grids::build_grid(UNSOLVED);
    assert_eq!(result.find_potentials(0, 0), vec![Some(3), Some(8)]);
}

#[test]
fn it_finds_no_potentials_in_a_cell() {
    let result = grids::build_grid(RESOLVED);
    assert_eq!(result.find_potentials(0, 0), vec![]);
}