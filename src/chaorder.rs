// Work in progress.
// TBC = To Be Configured.

use crossterm::{self, cursor, terminal};

use rand::prelude::*;

use std::{
    collections::{HashMap, HashSet},
    io, thread,
    time::Duration,
};

pub type Illustration = HashMap<(u16, u16), char>;

#[derive(Debug)]
pub struct TerminalCell {
    range_width_coefficient: u16,
    row: u16,
    column: u16,
    required_char: char,
}

impl TerminalCell {
    fn new(row: u16, column: u16, required_char: char) -> Self {
        Self {
            range_width_coefficient: 1,
            row,
            column,
            required_char,
        }
    }
}

pub struct TerminalDrawBoard {
    cells: Vec<TerminalCell>,
}

impl From<Illustration> for TerminalDrawBoard {
    fn from(illustration: Illustration) -> Self {
        let mut cells: Vec<TerminalCell> = Vec::with_capacity(illustration.len());

        for k in illustration.keys() {
            cells.push(TerminalCell::new(k.1, k.0, *illustration.get(k).unwrap()));
        }

        Self { cells }
    }
}

impl TerminalDrawBoard {
    pub fn draw(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        crossterm::execute!(
            io::stdout(),
            terminal::EnterAlternateScreen,
            terminal::Clear(terminal::ClearType::All)
        )?;
        let mut rng = thread_rng();

        // TODO: Bind `ready` cells register to `Self`.
        //       It shouldn't float on its own.
        let mut ready: HashSet<(u16, u16)> = HashSet::new();

        while ready.len() < self.cells.len() {
            // thread::sleep(Duration::from_nanos(120));
            thread::sleep(Duration::from_millis(1)); // TBC.

            let cell = self.cells.choose_mut(&mut rng).unwrap();

            if ready.contains(&(cell.row, cell.column)) {
                continue;
            }

            // TBC.
            let (lower, upper) = (
                (cell.required_char as u32)
                    .saturating_sub(50 / cell.range_width_coefficient as u32),
                // Panicking is considered unrealistic.
                cell.required_char as u32 + 50 / cell.range_width_coefficient as u32,
            );

            // Perhaps TBC.
            cell.range_width_coefficient *= 2;

            let generated_char = char::from_u32(rng.gen_range(lower..=upper)).unwrap();

            crossterm::execute!(io::stdout(), cursor::MoveTo(cell.row, cell.column))?;
            print!("{}", generated_char);

            if generated_char == cell.required_char {
                ready.insert((cell.row, cell.column));
            }
        }

        crossterm::execute!(
            io::stdout(),
            cursor::MoveTo(
                self.cells.iter().max_by_key(|cell| cell.row).unwrap().row as u16 + 1,
                self.cells
                    .iter()
                    .max_by_key(|cell| cell.column)
                    .unwrap()
                    .column as u16
                    + 1,
            ),
        )?;

        Ok(())
    }
}
