pub fn print_board(board: [[i32; 3]; 3]) {
    println!("  -------------");
    for line in board {
        print!("  |");
        for cell in line {
            print!(" {} |", cell);
        }
        println!();
        println!("  -------------");
    }
    println!("    1   2   3");
}

pub fn check_victory(board: [[i32; 3]; 3]) -> bool {
    /*check line and column alignment*/
    for line in board {
        if line[0] == line[1] && line[1] == line[2] && line[0] != 0 {
            return true;
        }
    }
    /*check diagonal alignments*/
    if board[0][0] == board[1][1] && board[1][1] == board[2][2] && board[0][0] != 0 {
        return true;
    }
    else {return false}
}