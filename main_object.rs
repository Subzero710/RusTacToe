use std::collections::HashMap;
use std::process::Command;
use std::io::{self, Write};


struct Board {
    grid: [[u8; 3]; 3],
}

impl Board{
    const SYMBOLS: [char; 3] = [' ', 'X', 'O'];

    fn new() -> Self {
        Self { grid: [[0; 3]; 3] }
    }

   fn reinit(&mut self) {
        for row in &mut self.grid {
            row.fill(0);
        }
    }

    fn print(&self) {
        println!("  {}", "-".repeat(13));
        for (i, row) in self.grid.iter().enumerate() {
            let letter = (b'A' + i as u8) as char;
            println!(
                "{} | {} | {} | {} |",
                letter, 
                Self::SYMBOLS[row[0] as usize], 
                Self::SYMBOLS[row[1] as usize], 
                Self::SYMBOLS[row[2] as usize]
            );
            println!("  {}", "-".repeat(13));
        }
        println!("    1   2   3");
    }

    fn check_victory(&self) -> bool {
        // Check row and col
        for i in 0..3 {
            if (self.grid[i][0] != 0 && self.grid[i][0] == self.grid[i][1] && self.grid[i][1] == self.grid[i][2])
                || (self.grid[0][i] != 0 && self.grid[0][i] == self.grid[1][i] && self.grid[1][i] == self.grid[2][i])
            {
                return true;
            }
        }
        // Check diago
        if (self.grid[0][0] != 0 && self.grid[0][0] == self.grid[1][1] && self.grid[1][1] == self.grid[2][2])
            || (self.grid[0][2] != 0 && self.grid[0][2] == self.grid[1][1] && self.grid[1][1] == self.grid[2][0])
        {
            return true;
        }
        false
    }
}


struct TicTacToe {
    turn: u8,
    board: Board,
}

impl TicTacToe{
   fn new() -> Self {
        Self {
            turn: 1,
            board: Board::new(),
        }
    } 

    fn reinit(&mut self) {
        self.turn = 1;
        self.board.reinit();
    }

    fn launch(&mut self) -> io::Result<()> {
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
        Command::new("clear").status().unwrap();
        loop{
            loop{
                let player = if self.turn % 2 == 1 { "X" } else { "O" };
                println!("{}", "*".repeat(20));
                println!("Turn {} : Player {}", self.turn, player);
                println!("{}", "*".repeat(20));
                self.board.print();

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
                if self.board.grid[row][col] != 0 {
                    Command::new("clear").status().unwrap();
                    println!("Square {} already used !", input);
                    continue;
                }
                self.board.grid[row][col] = 2 - (self.turn % 2);
                if self.turn >= 5 && self.board.check_victory() {
                    Command::new("clear").status().unwrap();
                    println!("Player {} won !", player);
                    self.board.print();
                    break;
                }
                if self.turn >= 9 {
                    Command::new("clear").status().unwrap();
                    println!("The game is draw !");
                    self.board.print();
                    break;
                }
                self.turn += 1;
                Command::new("clear").status().unwrap();
            }
            // Ask for another game
            print!("Do you want to play another game ? (Y/N) ");
            io::stdout().flush().unwrap();
            let mut another_game = String::new();
            if io::stdin().read_line(&mut another_game)? == 0 {
                break;
            }
            if another_game.trim().eq_ignore_ascii_case("Y") {
                self.reinit();
                Command::new("clear").status().unwrap();
            } else {
                break;
            }
        }
        Ok(())
    }


}


fn main() -> io::Result<()> {
    let mut game = TicTacToe::new();
    game.launch()
}