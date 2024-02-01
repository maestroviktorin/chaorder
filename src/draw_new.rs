use crate::illustration::Illustration;

use rand::prelude::*;

use ratatui::{
    buffer::Buffer,
    layout::Rect,
    widgets::Widget,
};

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
    fn render(self, _: Rect, _: &mut Buffer) {
        todo!()
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
    pub fn with_illustration(mut self, illustration: Illustration) -> Self {
        self.illustration = illustration;
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

    pub fn build(self) -> Illustrator {
        Illustrator {
            illustration: self.illustration,
            sleep_nanos: self.sleep_nanos,
            char_range: self.char_range,
            char_range_reduction_factor: self.char_range_reduction_factor,
        }
    }
}
