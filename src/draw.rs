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

pub struct Drawer {
    cells: Vec<TerminalCell>,
    start: (u16, u16),
    sleep_nanos: u64,
    char_range: u32,
    char_range_reduction_factor: u32,
}

impl Drawer {
    pub fn new(illustration: Illustration) -> DrawerBuilder {
        DrawerBuilder::from(illustration)
    }

    pub fn run(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        crossterm::execute!(
            io::stdout(),
            // These two commands must be somehow arranged for a real usage.
            terminal::EnterAlternateScreen,
            terminal::Clear(terminal::ClearType::All)
        )?;
        let mut rng = thread_rng();

        crossterm::execute!(io::stdout(), cursor::MoveTo(self.start.1, self.start.0))?;

        // Storing `(u16, u16)`s instead of `TerminalCell`s is cheaper
        // and does not require other overheads associated with mutual exclusion
        // and `derive`s for `TerminalCell`.
        //
        // Also, accounting for "ready" `TerminalCell`s
        // is a functionality related only to the `draw` method.
        let mut ready: HashSet<(u16, u16)> = HashSet::with_capacity(self.cells.len());

        while ready.len() < self.cells.len() {
            thread::sleep(Duration::from_nanos(self.sleep_nanos));

            let cell = self.cells.choose_mut(&mut rng).unwrap();

            if ready.contains(&(cell.row, cell.column)) {
                continue;
            }

            let (lower, upper) = (
                // Panicking is possible when working with relatively "small" `char`s.
                (cell.required_char as u32)
                    .saturating_sub(self.char_range / cell.range_width_coefficient),
                // Panicking is considered unrealistic.
                (cell.required_char as u32) + self.char_range / cell.range_width_coefficient,
            );

            cell.range_width_coefficient *= self.char_range_reduction_factor;

            let generated_char = char::from_u32(rng.gen_range(lower..=upper)).unwrap();

            crossterm::execute!(
                io::stdout(),
                cursor::MoveTo(self.start.1 + cell.row, self.start.0 + cell.column)
            )?;
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

impl From<DrawerBuilder> for Drawer {
    fn from(draw_builder: DrawerBuilder) -> Self {
        Self {
            cells: draw_builder.cells,
            start: draw_builder.start,
            sleep_nanos: draw_builder.sleep_nanos,
            char_range: draw_builder.char_range,
            char_range_reduction_factor: draw_builder.char_range_reduction_factor,
        }
    }
}

pub struct DrawerBuilder {
    cells: Vec<TerminalCell>,
    start: (u16, u16),
    sleep_nanos: u64,
    char_range: u32,
    char_range_reduction_factor: u32,
}

impl Default for DrawerBuilder {
    fn default() -> Self {
        Self {
            cells: Vec::new(),
            start: Default::default(),
            sleep_nanos: SLEEP_NANOS_DEFAULT,
            char_range: CHAR_RANGE_DEFAULT,
            char_range_reduction_factor: CHAR_RANGE_REDUCTION_FACTOR_DEFAULT,
        }
    }
}

impl From<Illustration> for DrawerBuilder {
    fn from(illustration: Illustration) -> Self {
        Self {
            cells: illustration
                .keys()
                .map(|key| TerminalCell::new(key.1, key.0, *illustration.get(key).unwrap()))
                .collect(),
            ..Default::default()
        }
    }
}

impl DrawerBuilder {
    pub fn with_start(mut self, start: (u16, u16)) -> Self {
        self.start = start;
        self
    }

    pub fn with_sleep_nanos(mut self, sleep_nanos: u64) -> Self {
        self.sleep_nanos = sleep_nanos;
        self
    }

    pub fn with_char_range(mut self, char_range: u32) -> Self {
        self.char_range = char_range;
        self
    }

    pub fn with_char_range_reduction_factor(mut self, char_range_reduction_factor: u32) -> Self {
        self.char_range_reduction_factor = char_range_reduction_factor;
        self
    }

    pub fn build(self) -> Drawer {
        Drawer {
            cells: self.cells,
            start: self.start,
            sleep_nanos: self.sleep_nanos,
            char_range: self.char_range,
            char_range_reduction_factor: self.char_range_reduction_factor,
        }
    }
}
