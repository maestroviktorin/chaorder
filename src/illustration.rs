//! A module that provides [`Illustration`] type alias
//! and related [`ParseIllustration`] functionality for parsing it from source data.

use std::{collections::HashMap, fs::read_to_string, io, path::Path};

/// Represents an illustration/text as a key-value pairs:  
/// `(row, column): character`  
///
/// Named `Illustration` for the sake of universality.
pub type Illustration = HashMap<(u16, u16), char>;

/// Provides methods for parsing [`Illustration`]s from source data.
pub struct ParseIllustration;

impl ParseIllustration {
    /// Parses an [`Illustration`] from a string slice.    
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

    /// Parses an [`Illustration`] from a `.txt` file.
    pub fn from_txt(path: &Path) -> io::Result<Illustration> {
        Ok(Self::from_str(&read_to_string(path)?))
    }
}
