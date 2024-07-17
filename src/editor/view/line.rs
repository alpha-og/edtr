use std::ops::Range;
use unicode_segmentation::UnicodeSegmentation;
use unicode_width::UnicodeWidthStr;

#[derive(Clone, Copy)]
pub enum GraphemeWidth {
    Half,
    Full,
}

pub struct Grapheme {
    content: String,
    width: GraphemeWidth,
    replacement: Option<char>,
}

pub struct Line {
    graphemes: Vec<Grapheme>,
}

impl Line {
    pub fn from(text: &str) -> Self {
        let graphemes: Vec<Grapheme> = text
            .graphemes(true)
            .map(|grapheme| Grapheme {
                content: grapheme.to_string(),
                width: match grapheme.width() {
                    0 | 1 => GraphemeWidth::Half,
                    _ => GraphemeWidth::Full,
                },
                replacement: match grapheme.width() {
                    0 => Some('·'),
                    _ => None,
                },
            })
            .collect();
        Self { graphemes }
    }
    pub fn get_visible_graphemes(&self, range: Range<usize>) -> String {
        let mut visible_graphemes = String::new();

        if range.start >= range.end {
            return visible_graphemes;
        }

        let mut current_position: usize = 0;

        for grapheme in &self.graphemes {
            let grapheme_width = match grapheme.width {
                GraphemeWidth::Half => 1,
                GraphemeWidth::Full => 2,
            };
            let grapheme_end = current_position.saturating_add(grapheme_width);

            if current_position >= range.end {
                break;
            }
            if grapheme_end > range.start {
                if current_position < range.start || grapheme_end > range.end {
                    visible_graphemes.push_str("⋯");
                } else if let Some(char) = grapheme.replacement {
                    visible_graphemes.push(char);
                } else {
                    visible_graphemes.push_str(&grapheme.content);
                }
            }
            current_position = grapheme_end;
        }
        visible_graphemes
    }
    pub fn grapheme_count(&self) -> usize {
        self.graphemes.len()
    }
    pub fn width_until(&self, grapheme_index: usize) -> usize {
        self.graphemes
            .iter()
            .take(grapheme_index)
            .map(|grapheme| match grapheme.width {
                GraphemeWidth::Half => 1,
                GraphemeWidth::Full => 2,
            })
            .sum()
    }
}
