//! A module that provides [`Drawer`] and related functionality. Uses `crossterm`.

use crate::illustration::Illustration;

use crossterm::{self, cursor, terminal};

use rand::prelude::*;

use std::{
    collections::{HashMap, HashSet},
    io, thread,
    time::Duration,
};

const SLEEP_NANOS_DEFAULT: u64 = 30;
const CHAR_RANGE_DEFAULT: u32 = 200;
const CHAR_RANGE_REDUCTION_FACTOR_DEFAULT: u32 = 2;

/// A cell on the terminal grid.  
///
/// **Fields description**:  
/// * `row`: The row coordinate.  
/// * `column`: The column coordinate.     
/// * `required_char`: The required character for the cell.
///   If this character was randomly selected, the terminal cell will not be touched anymore.  
#[derive(Debug, PartialEq, Eq, Hash)]
pub struct TerminalCell {
    row: u16,
    column: u16,
    required_char: char,
}

impl TerminalCell {
    /// Creates a [`TerminalCell`].  
    ///
    /// **Parameters description**:  
    /// * `row`: Row coordinate.  
    /// * `column`: Column coordinate.  
    /// * `required_char`: Required character for the cell.
    ///   If this character was randomly selected, the terminal cell will not be touched anymore.
    pub fn new(row: u16, column: u16, required_char: char) -> Self {
        Self {
            row,
            column,
            required_char,
        }
    }
}

/// A struct that handles the drawing process.  
///
/// **Fields description**:  
/// * `cells`: [Vector](Vec) of [`TerminalCell`]s. Can be thought as a "drawing board".
/// * `start`: Starting position of the drawing. The leftmost and uppermost corner.
/// *The following fields affect the drawing process speed*
/// * `sleep_nanos`: Sleep time in nanoseconds between each random cursor movement.
/// * `char_range`: All possible numeric values of randomly selected characters lie on the `(required_char - char_range; required_char + char_range)` interval.
/// * `char_range_reduction_factor`: Exponential reduction factor of the characters range (so-called "guessing-vicinity").
pub struct Drawer {
    cells: Vec<TerminalCell>,
    start: (u16, u16),
    sleep_nanos: u64,
    char_range: u32,
    char_range_reduction_factor: u32,
}

impl Drawer {
    /// Creates a [`DrawerBuilder`].  
    ///
    /// **Parameters description**  
    /// `illustration`: [`Illustration`] to be drawn by the upcoming [`Drawer`].
    pub fn new(illustration: Illustration) -> DrawerBuilder {
        DrawerBuilder::default().with_cells(illustration)
    }

    /// Starts the drawing process.  
    /// The drawing process can be configured when creating a [`Drawer`] with [`DrawerBuilder`].
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

        let mut range_width_coefficients: HashMap<&TerminalCell, u32> =
            HashMap::from_iter(self.cells.iter().map(|cell| (cell, 1)));

        while ready.len() < self.cells.len() {
            thread::sleep(Duration::from_nanos(self.sleep_nanos));

            let cell = self.cells.choose(&mut rng).unwrap();

            if ready.contains(&(cell.row, cell.column)) {
                continue;
            }

            let (lower, upper) = (
                // Panicking is possible when working with relatively "small" `char`s.
                (cell.required_char as u32)
                    .saturating_sub(self.char_range / range_width_coefficients.get(cell).unwrap()),
                // Panicking is considered unrealistic.
                (cell.required_char as u32)
                    + self.char_range / range_width_coefficients.get(cell).unwrap(),
            );

            *range_width_coefficients.get_mut(cell).unwrap() *= self.char_range_reduction_factor;

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
    /// Converts a [`DrawerBuilder`] into a [`Drawer`].
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

/// A builder for [`Drawer`].
///
/// See [`Drawer`]'s details.
pub struct DrawerBuilder {
    cells: Vec<TerminalCell>,
    start: (u16, u16),
    sleep_nanos: u64,
    char_range: u32,
    char_range_reduction_factor: u32,
}

impl Default for DrawerBuilder {
    // Auto-documented.
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

impl DrawerBuilder {
    /// Specifies the `cells` field of the upcoming [`Drawer`].  
    ///
    /// `cells` can be thought as an [`Illustration`] (which is actually a regular [`HashMap`])
    /// turned into a [vector](Vec).
    pub fn with_cells(mut self, illustration: Illustration) -> Self {
        self.cells = illustration
            .keys()
            .map(|key| TerminalCell::new(key.1, key.0, *illustration.get(key).unwrap()))
            .collect();
        self
    }

    /// Specifies the `start` field of the upcoming [`Drawer`].
    ///
    /// `start` is the starting coordinates of an upcoming drawing.  
    /// In other words, the leftmost and uppermost [`TerminalCell`].
    pub fn with_start(mut self, start: (u16, u16)) -> Self {
        self.start = start;
        self
    }

    /// Specifies the `sleep_nanos` field of the upcoming [`Drawer`].  
    ///
    /// `sleep_nanos` is the number of nanoseconds during which the thread
    /// will sleep between each random cursor movement.
    pub fn with_sleep_nanos(mut self, sleep_nanos: u64) -> Self {
        self.sleep_nanos = sleep_nanos;
        self
    }

    /// Specifies the `char_range` field of the upcoming [`Drawer`].  
    ///
    /// The numeric values of randomly selected characters
    /// are within the `(required_char - char_range; required_char + char_range)` interval.  
    ///
    /// See `with_char_range_reduction_factor`'s description for an example.
    pub fn with_char_range(mut self, char_range: u32) -> Self {
        self.char_range = char_range;
        self
    }

    /// Specifies the `char_range_reduction_factor` field of the upcoming [`Drawer`].
    ///
    /// After each unsuccessful attempt to guess the `required_char` of a [`TerminalCell`],
    /// its "guessing-vicinity" is narrowed `char_range_reduction_factor` times.  
    ///
    /// **Example**  
    /// Let `required_char` of the [`TerminalCell`] is `'A'` whose numeric value is `65`.   
    /// Let `char_range` is `30`.  
    /// Let **`char_range_reduction_factor`** is **`2`**.  
    ///
    /// First time the `Drawer` will try to guess `'A'` from `(65 - 30/2^0; 65 + 30/2^0)`, e.g. `(35; 95)`.  
    /// Let it didn't. 😞   
    /// Second time the `Drawer` will try to guess `'A'` from `(65 - 30/2^1; 65 + 30/2^1)`, e.g. `(50; 80)`.  
    /// Let it didn't again. 😖  
    /// Third time the `Drawer` will try to guess `'A'` from `(65 - 30/2^2; 65 + 30/2^2)`, e.g. `(65; 65)`.  
    /// And now it definitely did! 🎊
    pub fn with_char_range_reduction_factor(mut self, char_range_reduction_factor: u32) -> Self {
        self.char_range_reduction_factor = char_range_reduction_factor;
        self
    }

    /// Stops building and returns [`Drawer`] with all the preliminarily specified fields.
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
