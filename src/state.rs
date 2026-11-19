use std::collections::HashMap;
use std::sync::LazyLock;
pub type Move = HashMap<&'static str, (usize, usize)>;
pub static MOVES: LazyLock<Move> = LazyLock::new(|| {
    HashMap::from([
        ("A1", (0, 0)),
        ("A2", (0, 1)),
        ("A3", (0, 2)),
        ("B1", (1, 0)),
        ("B2", (1, 1)),
        ("B3", (1, 2)),
        ("C1", (2, 0)),
        ("C2", (2, 1)),
        ("C3", (2, 2)),
    ])
});
pub static TURN :i32 = 1;
pub static BOARD: [[i32;3]; 3] = [[0, 0, 0], [0, 0, 0], [0, 0, 0]];
pub const CHAR: [char;3] = [' ', 'X', 'O'];
pub const PLAYERS: [char;2] = ['X', 'O'];
