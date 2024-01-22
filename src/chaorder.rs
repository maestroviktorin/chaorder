use crossterm::{
    cursor, execute,
    terminal::{Clear, ClearType},
};

use rand::prelude::*;

use std::{collections::HashMap, io};

// To be deleted.
pub fn diagonal_print() {
    let mut row = 0;
    let mut col = 0;

    execute!(io::stdout(), Clear(ClearType::All)).unwrap();
    for _ in 0..10 {
        execute!(io::stdout(), cursor::MoveTo(row, col)).unwrap();
        print!("{}", row + col);
        row += 1;
        col += 1;
    }
    execute!(io::stdout(), cursor::MoveToNextLine(1)).unwrap();
}

#[derive(Debug, Hash)]
pub struct TerminalCell {
    row: u16,
    column: u16,
    required_char: char,
}

impl TerminalCell {
    fn new(row: u16, column: u16, required_char: char) -> Self {
        Self {
            row,
            column,
            required_char,
        }
    }
}

#[allow(dead_code)]
pub struct TerminalDrawBoard {
    // Can be useful in the future.
    rows_start: u16,
    columns_start: u16,
    rows_end: u16,
    columns_end: u16,

    cells: Vec<TerminalCell>,
}

impl TerminalDrawBoard {
    pub fn new(
        rows_start: u16,
        columns_start: u16,
        rows_end: u16,
        columns_end: u16,
        required_chars: HashMap<(u16, u16), char>,
    ) -> Self {
        let (rows, columns) = (rows_end - rows_start, columns_end - columns_start);
        let mut cells: Vec<TerminalCell> = Vec::with_capacity((rows * columns) as usize);

        for row in rows_start..rows_end {
            for col in columns_start..columns_end {
                cells.push(TerminalCell::new(
                    row,
                    col,
                    required_chars.get(&(row, col)).unwrap().to_owned(),
                ))
            }
        }

        Self {
            rows_start,
            columns_start,
            rows_end,
            columns_end,
            cells,
        }
    }

    // WIP.
    pub fn draw(&self) {
        execute!(io::stdout(), crossterm::terminal::EnterAlternateScreen).unwrap();
        let mut rng = thread_rng();

        let changed = true;
        while changed {
            let cell = self.cells.choose(&mut rng).unwrap();

            execute!(io::stdout(), cursor::MoveTo(cell.row, cell.column)).unwrap();
            print!("{}", char::from_u32(rng.gen_range(100..500)).unwrap())
        }
    }
}
