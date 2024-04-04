const BOARD_HEIGHT: usize = 16;
const BOARD_WIDTH: usize = 16;

fn generate_board(starting_locations: Option<&[(usize, usize)]>) -> Vec<Vec<bool>> {
    //generate the board using the defined constants above as the board width and height.
    //alive cells are set to locations specified in hw8 description
    let mut board = vec![vec![false; BOARD_WIDTH]; BOARD_HEIGHT];
    if let Some(locations) = starting_locations {
        for &(row, col) in locations {
            if row < BOARD_HEIGHT && col < BOARD_WIDTH {
                board[row][col] = true;
            }
        }
    }
    board
}

fn display_board(board: &Vec<Vec<bool>>) {
    //display the board, if a cell is true show 'o', if a cell is false show 'x'
    for row in board {
        for &cell in row {
            if cell {
                print!("* ");
            } else {
                print!(". ");
            }
        }
        println!();
    }
}

fn iteration(current_board: &Vec<Vec<bool>>) -> Vec<Vec<bool>> {
    //this will take the current board and run one generation of the game of life, then return a new board
    //that is the board after one generation
    let mut new_board = vec![vec![false; BOARD_WIDTH]; BOARD_HEIGHT];
    for row in 0..BOARD_HEIGHT {
        for col in 0..BOARD_WIDTH {
            new_board[row][col] = calculate_liveness(current_board, row, col);
        }
    }
    new_board
} 

fn calculate_liveness(board: &Vec<Vec<bool>>, row: usize, col: usize) -> bool {
    //this will calculate the liveness of the current cell, and the amount of live neighbors it has,
    //given the board and indexes of current cell
    let mut count = 0;
    //look at each possible neighbor, to do this iterate through a range decreasing row by 1 or increasing by 1, and the same with columns
    for i in -1..=1 {
        for j in -1..=1 {
            //skip the cell itself
            if i == 0 && j == 0 {continue}
            let neighbor_row = (row as isize + i + BOARD_HEIGHT as isize) % BOARD_HEIGHT as isize;
            let neighbor_col = (col as isize + j + BOARD_WIDTH as isize) % BOARD_WIDTH as isize;
            if board[neighbor_row as usize][neighbor_col as usize] {count += 1}
        }
    }
    if board[row][col] {
        //we have a live cell, if the number of alive neighbors is anything but 2 or 3, the cell dies
        if count < 2 || count > 3 {
            false
        } else {
            true
        }
    } else {
        //we have a dead cell
        if count == 3 {
            true
        }
        else {
            false
        }
    }
}

fn main() {
    let starting_locations = &[(0, 1), (1, 2), (2, 0), (2, 1), (2, 2)];
    let mut board = generate_board(Some(starting_locations));
    for i in 0..10 {
        println!("Iteration {}:", i+1);
        display_board(&board);
        println!();
        board = iteration(&board);
    }
}

#[test]
fn test_liveness_calc_corner() {
    //this will test that the calculate_liveness function is working correctly for a cell placed in a corner
    let starting_locations = &[(0,0)];
    let mut board = generate_board(Some(starting_locations));
    let alive = calculate_liveness(&board, 0, 0);
    assert_eq!(alive, false, "Cell is persistent! thats an issue...");
}