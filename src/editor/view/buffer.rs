use std::{fs::read_to_string, io::Error};

use super::line::Line;

#[derive(Default)]
pub struct Buffer {
    pub lines: Vec<Line>,
}

impl Buffer {
    pub fn load(file: &str) -> Result<Self, Error> {
        let contents = read_to_string(file)?;

        let mut lines = Vec::new();
        for v in contents.lines() {
            lines.push(Line::from(v));
        }

        Ok(Self { lines })
    }

    pub fn is_empty(&self) -> bool {
        self.lines.is_empty()
    }
}
