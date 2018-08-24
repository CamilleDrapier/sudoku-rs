pub mod models;

pub type Sudoku = [[Option<u8>; 9]; 9];

pub fn solve(sud: Sudoku) -> Sudoku {
    let mut prev_number = 0;
    let mut grid = models::grids::build_grid(sud);;
    while grid.number_found < (9 * 9) && grid.number_found > prev_number {
        prev_number = grid.number_found;
        for i in 0..9 {
            for j in 0..9 {
                if grid.get_value(i, j).is_none() {
                    let potentials = grid.find_potentials(i, j);
                    if potentials.len() == 1 {
                        //println!("Found [{}][{}] -> {:?}", i, j, *potentials.first().unwrap());
                        grid.set_value(i, j,*potentials.first().unwrap());
                    }else {
                        for potential in potentials.iter() {
                            if grid.get_value(i, j).is_none() && check_impossible(&grid, i, j, potential) {
                                //println!("Found Impossible [{}][{}] -> {:?}", i, j, potential.unwrap());
                                grid.set_value(i, j,*potential);
                            }
                        }
                    }
                }
            }
        }
    }
    *grid.to_sudoku()
}

pub fn check_impossible(grid: &models::grids::Grid, i: usize, j: usize, candidate: &Option<u8>) -> bool {
    let mut line_impossible = true;
    for (x, cell) in grid.sudoku[i].iter().enumerate() {
        if x != j {
            line_impossible = line_impossible &&
                (
                    cell.is_some() ||
                        grid.line(i).contains(candidate) ||
                        grid.column(x).contains(candidate) ||
                        grid.region(i, x).contains(candidate)
                )
        }
    }
    let mut column_impossible = true;
    for (x, line) in grid.sudoku.iter().enumerate() {
        if x != i {
            for (y, cell) in line.iter().enumerate() {
                if y == j {
                    column_impossible = column_impossible &&
                        (
                            cell.is_some() ||
                                grid.line(x).contains(candidate) ||
                                grid.column(y).contains(candidate) ||
                                grid.region(x, y).contains(candidate)
                        )
                }
            }
        }
    }
    line_impossible || column_impossible
}