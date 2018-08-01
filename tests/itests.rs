extern crate sudoku;

#[test]
fn it_inits_all_none() {
    let grid = sudoku::models::grids::build_grid([[None; 9]; 9]);
    assert_eq!(grid.value[0][0].value, None);
    assert_eq!(grid.value[8][8].value, None);
}

#[test]
fn it_solves() {
    let expected = [
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
    let input = [
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
    let result = sudoku::solve(input);
    assert_eq!(expected, result);
}

#[test]
fn it_returns_a_solved_grid() {
    let expected = [
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

    let result = sudoku::solve(expected);
    assert_eq!(expected, result);
}

#[test]
fn it_inits_an_empty_cell() {
    let cell = sudoku::models::cells::build_cell();
    assert_eq!(cell.value, None);
}

#[test]
fn it_finds_potentials_in_an_empty_cell() {
    let cell = sudoku::models::cells::build_cell();
    assert_eq!(cell.find_potentials(), vec![Some(1), Some(2), Some(3), Some(4), Some(5), Some(6), Some(7), Some(8), Some(9)]);
}

#[test]
fn it_finds_potentials_in_a_cell() {
    let mut cell = sudoku::models::cells::build_cell();
    cell.line[3] = Some(4);
    cell.line[5] = Some(9);
    cell.column[1] = Some(9);
    cell.column[6] = Some(6);
    cell.region[3] = Some(7);
    cell.region[0] = Some(1);
    assert_eq!(cell.find_potentials(), vec![Some(2), Some(3), Some(5), Some(8)]);
}

#[test]
fn it_finds_no_potentials_in_a_cell() {
    let mut cell = sudoku::models::cells::build_cell();
    cell.line[3] = Some(1);
    cell.line[5] = Some(2);
    cell.line[8] = Some(7);
    cell.column[1] = Some(3);
    cell.column[6] = Some(4);
    cell.column[0] = Some(8);
    cell.region[3] = Some(5);
    cell.region[0] = Some(6);
    cell.region[2] = Some(9);
    assert_eq!(cell.find_potentials(), vec![]);
}