use std::fmt;

const SIZE: usize = 9; //in Rust, you need usize to index arrays and slicing. Remember that in
                       //contigous memory, the memory offset is dependent on system's architecture.
                       //usize infers the system architecture and calculates the proper offset. 
const EMPTY: u8 = 0; //we use 0 to represent empty cells since 0 is not a valid input in sudoku.


//in Rust, struct are used to create custom data types. To add methods to this, use impl.
struct SudokuSolver {
    grid: [[u8; SIZE]; SIZE],
}

impl SudokuSolver {

    fn new(grid: [[u8; SIZE]; SIZE]) -> Self {
        Self {grid}
    }

    
    //self has to be mutable here because we're updating the grid. In other functions, we're not
    //modifying any self property.
    fn solve(&mut self) -> bool {
       //basically if the find_empty() returns the "SOME", then the code executes.
       //since self.find_empty() returns Option<T>, if statement can't handle it. Hence the
       //workaround.
       if let Some((row, col)) = self.find_empty() {
            for num in 1..=9 {
                if self.is_valid(row, col, num) {
                    self.grid[row][col] = num;

                    if self.solve() {
                        return true;
                    }

                    self.grid[row][col] = EMPTY; //backtracking
                }
            }
            return false; //if there is an empty cell but no valid number, this triggers
                          //backtracking.
       }
       true
    }


    fn find_empty(&self) -> Option<(usize, usize)> {
        
        //By default, a range (0..n) in Rust produces values of type usize because it is designed for indexing operations.
        for row in 0..SIZE {
            for col in 0..SIZE {
                if self.grid[row][col] == EMPTY {
                    return Some((row, col));
                }
            }
        }
        None
    }

    fn is_valid(&self, row:usize, col:usize, num: u8) -> bool {
        
        //check row-wise
        for c in 0..SIZE {
            if self.grid[row][c] == num {
                return false;
            }
        }

        //check col-wise
        for r in 0..SIZE {
            if self.grid[r][col] == num {
                return false;
            }
        }

        //check the grid
        let box_start_row = row / 3 * 3;
        let box_start_col = col / 3 * 3;

        for r in box_start_row..box_start_row+3 {
            for c in box_start_col..box_start_col+3 {
                if self.grid[r][c] == num {
                    return false;
                }
            }
        }

        true
    }


    fn display(&self) {
        for row in 0..SIZE {
            if row % 3 == 0 && row != 0 {
                println!("------+------+------");
            }

            for col in 0..SIZE {
                if col % 3 == 0 && col != 0 {
                    print!("| ");
                }
                if self.grid[row][col] == EMPTY {
                    print!(". ");
                } else {
                    print!("{} ", self.grid[row][col]);
                }
            }
            println!();
        }
    }

}


// Implement Display for better printing
impl fmt::Display for SudokuSolver {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in 0..SIZE {
            if row % 3 == 0 && row != 0 {
                writeln!(f, "------+-------+------")?;
            }
            for col in 0..SIZE {
                if col % 3 == 0 && col != 0 {
                    write!(f, "| ")?;
                }
                if self.grid[row][col] == EMPTY {
                    write!(f, ". ")?;
                } else {
                    write!(f, "{} ", self.grid[row][col])?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}



fn main() {

    let puzzle = [
        [5, 3, 0, 0, 7, 0, 0, 0, 0],
        [6, 0, 0, 1, 9, 5, 0, 0, 0],
        [0, 9, 8, 0, 0, 0, 0, 6, 0],
        [8, 0, 0, 0, 6, 0, 0, 0, 3],
        [4, 0, 0, 8, 0, 3, 0, 0, 1],
        [7, 0, 0, 0, 2, 0, 0, 0, 6],
        [0, 6, 0, 0, 0, 0, 2, 8, 0],
        [0, 0, 0, 4, 1, 9, 0, 0, 5],
        [0, 0, 0, 0, 8, 0, 0, 7, 9],
    ];

    let mut solver = SudokuSolver::new(puzzle);

    println!("Initial Sudoku Puzzle");
    solver.display();
    
    if solver.solve() {
        println!("\nSolved puzzle!");
        solver.display();
    } else {
        println!("No Solution exist for this puzzle!");
    }
}

