// Work in progress.

use crate::illustration::Illustration;

use crossterm::{self, cursor, terminal};

use rand::prelude::*;

use std::{collections::HashSet, io, thread, time::Duration};

const SLEEP_NANOS_DEFAULT: u64 = 30;
const CHAR_RANGE_DEFAULT: u32 = 200;
const CHAR_RANGE_REDUCTION_FACTOR_DEFAULT: u32 = 2;

#[derive(Debug)]
struct TerminalCell {
    row: u16,
    column: u16,
    range_width_coefficient: u32,
    required_char: char,
}

impl TerminalCell {
    fn new(row: u16, column: u16, required_char: char) -> Self {
        Self {
            row,
            column,
            range_width_coefficient: 1,
            required_char,
        }
    }
}

pub struct TerminalDrawBoard {
    cells: Vec<TerminalCell>,
}

impl From<Illustration> for TerminalDrawBoard {
    fn from(illustration: Illustration) -> Self {
        Self {
            cells: illustration
                .keys()
                .map(|key| TerminalCell::new(key.1, key.0, *illustration.get(key).unwrap()))
                .collect(),
        }
    }
}

impl TerminalDrawBoard {
    pub fn draw(
        &mut self,
        start: Option<&(u16, u16)>,
        sleep_nanos: Option<u64>,
        char_range: Option<u32>,
        char_range_reduction_factor: Option<u32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        crossterm::execute!(
            io::stdout(),
            terminal::EnterAlternateScreen,
            terminal::Clear(terminal::ClearType::All)
        )?;
        let mut rng = thread_rng();

        let start = start.unwrap_or(&(0, 0));
        crossterm::execute!(io::stdout(), cursor::MoveTo(start.1, start.0))?;

        // Storing `(u16, u16)`s instead of `TerminalCell`s is cheaper
        // and does not require other overheads associated with mutual exclusion
        // and `derive`s for `TerminalCell`.
        //
        // Also, accounting for "ready" `TerminalCell`s
        // is a functionality related only to the `draw` method.
        let mut ready: HashSet<(u16, u16)> = HashSet::with_capacity(self.cells.len());

        while ready.len() < self.cells.len() {
            thread::sleep(Duration::from_nanos(
                sleep_nanos.unwrap_or(SLEEP_NANOS_DEFAULT),
            ));

            let cell = self.cells.choose_mut(&mut rng).unwrap();

            if ready.contains(&(cell.row, cell.column)) {
                continue;
            }

            let (lower, upper) = (
                // Panicking is possible when working with relatively "small" `char`s.
                (cell.required_char as u32).saturating_sub(
                    char_range.unwrap_or(CHAR_RANGE_DEFAULT) / cell.range_width_coefficient,
                ),
                // Panicking is considered unrealistic.
                (cell.required_char as u32)
                    + char_range.unwrap_or(CHAR_RANGE_DEFAULT) / cell.range_width_coefficient,
            );

            cell.range_width_coefficient *=
                char_range_reduction_factor.unwrap_or(CHAR_RANGE_REDUCTION_FACTOR_DEFAULT);

            let generated_char = char::from_u32(rng.gen_range(lower..=upper)).unwrap();

            crossterm::execute!(
                io::stdout(),
                cursor::MoveTo(start.1 + cell.row, start.0 + cell.column)
            )?;
            print!("{}", generated_char);

            if generated_char == cell.required_char {
                ready.insert((cell.row, cell.column));
            }
        }

        // TODO: Refactor.
        // When using `draw` on the same `TerminalDrawBoard` instance multiple times, overflow may be caused.
        self.cells
            .iter_mut()
            .map(|cell| cell.range_width_coefficient = 1)
            .for_each(drop);

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
