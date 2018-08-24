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
                            if grid.get_value(i, j).is_none() && grid.check_impossible(i, j, potential) {
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