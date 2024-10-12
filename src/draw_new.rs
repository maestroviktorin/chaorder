use crate::illustration::Illustration;
use rand::prelude::*;
use ratatui::{buffer::Buffer, layout::Rect, widgets::{StatefulWidget, Widget}};
use std::{
    collections::{HashMap, HashSet},
    thread,
    time::Duration,
};

const SLEEP_NANOS_DEFAULT: u64 = 3000000;
const CHAR_RANGE_DEFAULT: u32 = 200;
const CHAR_RANGE_REDUCTION_FACTOR_DEFAULT: u32 = 2;

#[derive(Default)]
pub struct IllustratorState {
    pub ready_cells: HashSet<(u16, u16)>,
    range_width_coefficients: HashMap<(u16, u16), u32>,
}

impl From<(&Buffer, &Illustration)> for IllustratorState {
    fn from(tuple: (&Buffer, &Illustration)) -> Self {
        Self {
            ready_cells: HashSet::with_capacity(tuple.1.len()),
            range_width_coefficients: HashMap::from_iter((tuple.0.area().left()..tuple.0.area().right()).flat_map(|col| {
                (tuple.0.area().top()..tuple.0.area().bottom()).map(move |row| ((col, row), 1))
            }))
        }
    }
}

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

impl StatefulWidget for &Illustrator {
    type State = IllustratorState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let mut rng = thread_rng();

        thread::sleep(Duration::from_nanos(self.sleep_nanos));

        let (col, row) = (
            rng.gen_range(area.left()..area.right()) as u16,
            rng.gen_range(area.top()..area.bottom()),
        );

        if state.ready_cells.contains(&(col, row)) {
            return;
        }

        let required_char = self.illustration.get(&(row, col)).unwrap().to_owned();
        let abs_vicinity = self.char_range / state.range_width_coefficients.get(&(col, row)).unwrap();
        let (lower, upper) = (
            (required_char as u32).saturating_sub(abs_vicinity),
            (required_char as u32) + abs_vicinity,
        );
        let generated_char = char::from_u32(rng.gen_range(lower..=upper)).unwrap();

        *state.range_width_coefficients.get_mut(&(col, row)).unwrap() *=
            self.char_range_reduction_factor;

        if generated_char == required_char {
            state.ready_cells.insert((col, row));
        }

        buf.get_mut(col, row).set_char(generated_char);
    }
}

impl Widget for Illustrator {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let mut state = IllustratorState::from((&buf.to_owned(), &self.illustration));

        while state.ready_cells.len() < self.illustration.len() {
            StatefulWidget::render(&self, area, buf, &mut state)
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
