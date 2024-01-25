// Work in progress.

use std::{collections::HashMap, fs::read_to_string, path::Path};

pub type Illustration = HashMap<(u16, u16), char>;

pub struct ParseIllustration;

impl ParseIllustration {
    pub fn from_txt(path: &Path) -> Illustration {
        read_to_string(path)
            .unwrap()
            .lines()
            .enumerate()
            .flat_map(|(row, line)| {
                line.chars()
                    .enumerate()
                    .map(move |(column, chr)| ((row as u16, column as u16), chr))
            })
            .collect()
    }
}
