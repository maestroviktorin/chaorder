use ::chaorder::{chaorder::Illustration, *};

use std::collections::HashMap;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let illustration: Illustration = HashMap::from_iter(
        vec![
            ((0, 0), ' '),
            ((0, 1), ' '),
            ((0, 2), 'U'),
            ((0, 3), 'R'),
            ((0, 4), 'A'),
            ((0, 5), 'L'),
            ((0, 6), ' '),
            ((0, 7), ' '),
            ((0, 8), ' '),
            ((1, 0), ' '),
            ((1, 1), ' '),
            ((1, 2), ' '),
            ((1, 3), ' '),
            ((1, 4), '/'),
            ((1, 5), '\\'),
            ((1, 6), ' '),
            ((1, 7), ' '),
            ((1, 8), ' '),
            ((2, 0), ' '),
            ((2, 1), '/'),
            ((2, 2), '\\'),
            ((2, 3), '/'),
            ((2, 4), '\\'),
            ((2, 5), '\\'),
            ((2, 6), ' '),
            ((2, 7), '\\'),
            ((2, 8), ' '),
        ]
        .into_iter(),
    );

    let mut board = chaorder::TerminalDrawBoard::from(illustration);
    board.draw()
}
