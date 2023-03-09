use std::io;
use ndarray::{self, arr2, Array2, Array1};

fn main() {
    
    let stdin = io::stdin();
    let mut grid = arr2(&[
        ['-', '-', '-'],
        ['-', '-', '-'],
        ['-', '-', '-']
    ]);
    
    let mut x_turn: bool = true;
    let mut playing: bool = true;

    while playing {
        
        // Prompt player
        if x_turn {
            println!("X's Turn. Where would you like to play?");
        } else {
            println!("O's Turn. Where would you like to play?");
        }

        // Initialize position variables
        let mut x_str = String::new();
        let mut y_str = String::new();
        let x_num: usize;
        let y_num: usize;
        
        // Get position
        println!("Column: ");
        stdin.read_line(&mut x_str).expect("Read failed: at x_str");

        println!("Row: ");
        stdin.read_line(&mut y_str).expect("Read failed: at y_str");
        
        // x_num and y_num are set to the column/row minus 1 to get the indices
        x_num = x_str.trim().parse::<usize>().expect("Parse failed: at x_num") - 1;
        y_num = y_str.trim().parse::<usize>().expect("Parse failed: at y_num") - 1;

        // // Debug --------
        // println!("Column: {}, Row: {}", x_num+1, y_num+1);
        // println!("Column index: {x_num}, Row index: {y_num}");
        // // -------- Debug

        // If the cell is empty
        if grid[(y_num, x_num)] == '-' {
            // Place an X/O
            if x_turn {
                grid[(y_num, x_num)] = 'X';
            } else {
                grid[(y_num, x_num)] = 'O';
            }
        } else { // If the cell is invalid, the grid will not be printed and the turn will not change
            println!("Invalid location, please try again.");
            continue;
        }

        // Print the grid as a grid
        for i in 0..3 {
            for j in 0..3 {
                print!("{} ", grid[(i, j)]);
            }
            print!("\n");
        }

        // Check for a winner
        match check_win(&grid) {
            true => { 
                playing = false;
                println!("Someone has won!");
                continue;
            },
            false => ()
        };

        // Switch turn
        if x_turn {
            x_turn = false;
        } else {
            x_turn = true;
        }
    }
}

// 
fn check_win(grid: &Array2<char>) -> bool {
    // Check columns
    for col in grid.columns() {
        if col.iter().all(|n| n == &col[0]) && col[0] != '-' { 
            return true;
        }
    }

    // Check rows
    for row in grid.rows() {
        if row.iter().all(|n| n == &row[0]) && row[0] != '-' {
            return true;
        }
    }

    // Check diagonal
    if grid.diag().iter().all(|n| n == &grid.diag()[0]) && grid.diag()[0] != '-' {
        return true;
    }

    // Check other diagonal
    if diag_2(grid).iter().all(|n| n == &diag_2(grid)[0]) && diag_2(grid)[0] != '-' {
        return true;
    }

    // Return false if no one has won
    false
}

// Returns an Array1 containing the characters on the 'other diagonal' of the given Array2
fn diag_2(grid: &Array2<char>) -> Array1<char> {
    // Copy the given array
    let mut invert = grid.clone();
    // Invert the array side-to-side
    invert.invert_axis(ndarray::Axis(1));
    // Return the diagonal of the inverted array
    invert.diag().to_owned()
}