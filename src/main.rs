pub mod rules;
pub mod state;
use std::io::{stdin,stdout,Write};

fn main() {
    println!("{}", "*".repeat(20));
    println!("Turn {} : Player {}",state::TURN, state::PLAYERS[((state::TURN)%2) as usize]);
    println!("{}", "*".repeat(20));
    rules::print_board(state::BOARD);
    let mut moove=String::new();
    print!("Enter your move:");
    stdout().flush().unwrap();
    stdin().read_line(&mut moove).expect("Did not enter a correct string");
    if let Some('\n')=moove.chars().next_back() {
        moove.pop();
    }
    if let Some('\r')=moove.chars().next_back() {
        moove.pop();
    }
    if !state::MOVES.contains_key(moove.as_str()) {
        println!("is not an authorized move !");
    }
    let coord  = state::MOVES[moove.as_str()];
    if state::BOARD[coord.0][coord.1] == 0 {
        let _ = state::BOARD[coord.0][coord.1] == 2 - (state::TURN % 2);
    }
    else{
        println!("Square already used !");
    }
    if state::TURN>=5{
        if rules::check_victory(state::BOARD){
            println!("Player {} won !", state::TURN);
        }
    }
}
