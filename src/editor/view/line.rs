use std::{cmp, ops::Range};

pub struct Line {
    text: String,
}

impl Line {
    pub fn from(text: &str) -> Self {
        Self {
            text: String::from(text),
        }
    }

    pub fn get(&self, range: Range<usize>) -> String {
        let start = range.start;
        let end = cmp::min(range.end, self.text.len());

        self.text.get(start..end).unwrap_or_default().to_string()
    }
}
