use ::chaorder::*;

use std::collections::HashMap;

fn main() {
    let mut required_chars: HashMap<(u16, u16), char> = HashMap::new();
    for i in 0..100 {
        for j in 0.. 100 {
            required_chars.insert((i, j), 'a');
        }
    }

    let board = chaorder::TerminalDrawBoard::new(0, 0, 50, 15, required_chars);
    board.draw();
}
