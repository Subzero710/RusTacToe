use std::collections::HashMap;
use std::process::Command;
use std::io::{self, Write};

fn print_board(board: &[[u8; 3]; 3])
{
    let symbols = [' ', 'X', 'O'];
    println!("  {}", "-".repeat(13));
    for (i, row) in board.iter().enumerate() {
        let letter = (b'A' + i as u8) as char;
        println!(
            "{} | {} | {} | {} |",
            letter, 
            symbols[row[0] as usize], 
            symbols[row[1] as usize], 
            symbols[row[2] as usize]
        );
        println!("  {}", "-".repeat(13));
    }
    println!("    1   2   3");
}

fn check_victory(board: &[[u8; 3]; 3]) -> bool {
    // Check row and col
    for i in 0..3 {
        if (board[i][0] != 0 && board[i][0] == board[i][1] && board[i][1] == board[i][2])
            || (board[0][i] != 0 && board[0][i] == board[1][i] && board[1][i] == board[2][i])
        {
            return true;
        }
    }
    // Check diago
    if (board[0][0] != 0 && board[0][0] == board[1][1] && board[1][1] == board[2][2])
        || (board[0][2] != 0 && board[0][2] == board[1][1] && board[1][1] == board[2][0])
    {
        return true;
    }
    false   
}

fn main() {
    let moves = HashMap::from([
        ("A1", (0, 0)),
        ("A2", (0, 1)),
        ("A3", (0, 2)),
        ("B1", (1, 0)),
        ("B2", (1, 1)),
        ("B3", (1, 2)),
        ("C1", (2, 0)),
        ("C2", (2, 1)),
        ("C3", (2, 2)),
    ]);
    let mut turn = 1u8;
    let mut board = [[0u8; 3]; 3];
    Command::new("clear").status().unwrap();
    loop{
        let player = if turn % 2 == 1 { "X" } else { "O" };
        println!("{}", "*".repeat(20));
        println!("Turn {} : Player {}", turn, player);
        println!("{}", "*".repeat(20));
        print_board(&board);

        print!("Enter your move: ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        if io::stdin().read_line(&mut input).unwrap() == 0 {
            break;
        }
        let input = input.trim().to_uppercase();
        if !moves.contains_key(input.as_str()) {
            Command::new("clear").status().unwrap();
            println!("{} is not an authorized move !", input);
            continue;
        }
        let (row, col) = moves[input.as_str()];
        if board[row][col] != 0 {
            Command::new("clear").status().unwrap();
            println!("Square {} already used !", input);
            continue;
        }
        board[row][col] = 2 - (turn % 2);
        if turn >= 5 && check_victory(&board) {
            Command::new("clear").status().unwrap();
            println!("Player {} won !", player);
            print_board(&board);
            break;
        }
        if turn >= 9 {
            Command::new("clear").status().unwrap();
            println!("The game is draw !");
            print_board(&board);
            break;
        }
        turn += 1;
        Command::new("clear").status().unwrap();
    }
}
