// Work in progress.

use std::{collections::HashMap, fs::read_to_string, io, path::Path};

pub type Illustration = HashMap<(u16, u16), char>;

pub struct ParseIllustration;

impl ParseIllustration {
    pub fn from_str(source: &str) -> Illustration {
        source
            .lines()
            .enumerate()
            .flat_map(|(row, line)| {
                line.chars()
                    .enumerate()
                    .map(move |(column, chr)| ((row as u16, column as u16), chr))
            })
            .collect()
    }

    pub fn from_txt(path: &Path) -> io::Result<Illustration> {
        Ok(Self::from_str(&read_to_string(path)?))
    }
}
