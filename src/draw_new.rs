use crate::illustration::Illustration;

use rand::prelude::*;

use ratatui::{buffer::Buffer, layout::Rect, widgets::Widget};

use std::{
    collections::{HashMap, HashSet},
    thread,
    time::Duration,
};

const SLEEP_NANOS_DEFAULT: u64 = 30;
const CHAR_RANGE_DEFAULT: u32 = 200;
const CHAR_RANGE_REDUCTION_FACTOR_DEFAULT: u32 = 2;

pub struct Illustrator {
    illustration: Illustration,
    sleep_nanos: u64,
    char_range: u32,
    char_range_reduction_factor: u32,
}

impl Illustrator {
    pub fn new() -> IllustratorBuilder {
        IllustratorBuilder::default()
    }
}

impl Widget for Illustrator {
    fn render(self, _: Rect, buf: &mut Buffer) {
        let mut rng = thread_rng();

        // Storing `(u16, u16)`s instead of `Cell`s is cheaper
        // and does not require other overheads associated with mutual exclusion
        // and `derive`s for `Cell`.
        //
        // Also, accounting for "ready" `Cell`s
        // is a functionality related only to the `draw` method.
        let mut ready: HashSet<(u16, u16)> = HashSet::with_capacity(buf.area().area() as usize);

        // FIXME.
        let mut range_width_coefficients: HashMap<(u16, u16), u32> =
            HashMap::from_iter((buf.area().left()..buf.area().right()).flat_map(|col| {
                (buf.area().top()..buf.area().bottom()).map(move |row| ((col, row), 1))
            }));

        while ready.len() < self.illustration.len() {
            thread::sleep(Duration::from_nanos(self.sleep_nanos));

            let rect = buf.area();
            let (col, row) = (
                rng.gen_range(rect.left()..rect.right()) as u16,
                rng.gen_range(rect.top()..rect.bottom()),
            );

            if ready.contains(&(col, row)) {
                continue;
            }

            let required_char = self.illustration.get(&(row, col)).unwrap().to_owned();
            let abs_vicinity = self.char_range / range_width_coefficients.get(&(col, row)).unwrap();
            let (lower, upper) = (
                (required_char as u32).saturating_sub(abs_vicinity),
                (required_char as u32) + abs_vicinity,
            );
            let generated_char = char::from_u32(rng.gen_range(lower..=upper)).unwrap();

            *range_width_coefficients.get_mut(&(col, row)).unwrap() *=
                self.char_range_reduction_factor;

            if generated_char == required_char {
                ready.insert((col, row));
            }

            buf.get_mut(col, row).set_char(generated_char);
        }
    }
}

pub struct IllustratorBuilder {
    illustration: Illustration,
    sleep_nanos: u64,
    char_range: u32,
    char_range_reduction_factor: u32,
}

impl Default for IllustratorBuilder {
    fn default() -> Self {
        Self {
            illustration: Default::default(),
            sleep_nanos: SLEEP_NANOS_DEFAULT,
            char_range: CHAR_RANGE_DEFAULT,
            char_range_reduction_factor: CHAR_RANGE_REDUCTION_FACTOR_DEFAULT,
        }
    }
}

impl IllustratorBuilder {
    pub fn illustration(mut self, illustration: Illustration) -> Self {
        self.illustration = illustration;
        self
    }

    pub fn sleep_nanos(mut self, sleep_nanos: u64) -> Self {
        self.sleep_nanos = sleep_nanos;
        self
    }

    pub fn char_range(mut self, char_range: u32) -> Self {
        self.char_range = char_range;
        self
    }

    pub fn char_range_reduction_factor(mut self, char_range_reduction_factor: u32) -> Self {
        self.char_range_reduction_factor = char_range_reduction_factor;
        self
    }

    pub fn build(self) -> Illustrator {
        Illustrator {
            illustration: self.illustration,
            sleep_nanos: self.sleep_nanos,
            char_range: self.char_range,
            char_range_reduction_factor: self.char_range_reduction_factor,
        }
    }
}
