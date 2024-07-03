use std::{cmp, ops::Range};
use unicode_segmentation::UnicodeSegmentation;

pub struct Line {
    string: String,
}

impl Line {
    pub fn from(content: &str) -> Self {
        Self {
            string: String::from(content),
        }
    }
    pub fn get(&self, range: Range<usize>) -> String {
        let start = range.start;
        let end = cmp::min(range.end, self.string.len());
        self.string[..]
            .graphemes(true)
            .skip(start)
            .take(end - start)
            .collect()
    }
    pub fn len(&self) -> usize {
        self.string[..].graphemes(true).count()
    }
}
