mod buffer;
mod line;
mod location;

use std::cmp::min;

use buffer::Buffer;
use location::Location;

use super::{
    editorcommand::{Direction, EditorCommand},
    terminal::{Position, Size, Terminal},
};

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

pub struct View {
    buffer: Buffer,
    redraw: bool,
    size: Size,
    text_location: Location,
    scroll_offset: Position,
}

impl Default for View {
    fn default() -> Self {
        Self {
            buffer: Default::default(),
            redraw: true,
            size: Terminal::size().unwrap_or_default(),
            text_location: Default::default(),
            scroll_offset: Default::default(),
        }
    }
}

impl View {
    pub fn render(&mut self) {
        let Size { height, width } = self.size;
        if !self.redraw || height == 0 || width == 0 {
            return;
        }
        let top = self.scroll_offset.row;
        for row in 0..height {
            if let Some(line) = self.buffer.lines.get(row.saturating_add(top)) {
                let left = self.scroll_offset.col;
                let right = self.scroll_offset.col.saturating_add(width);
                Self::render_line(row, &line.get_visible_graphemes(left..right));
            } else if self.buffer.is_empty() && row == height / 3 {
                Self::render_line(row, &Self::generate_welcome_message(width));
            } else {
                Self::render_line(row, "~");
            }
        }
        self.redraw = false;
    }
    pub fn render_line(row: usize, content: &str) {
        let result = Terminal::print_row(row, content);
        debug_assert!(result.is_ok(), "Failed to render line");
    }
    fn generate_welcome_message(width: usize) -> String {
        let mut message = format!("Welcome to {NAME} -- version {VERSION}");
        let padding_left = (width.saturating_sub(message.len())) / 2;
        let space_padding = " ".repeat(padding_left);
        message = format!("~{space_padding}{message}");
        message
    }
    pub fn load(&mut self, file_path: &str) -> Result<(), std::io::Error> {
        if let Ok(buffer) = Buffer::load(file_path) {
            self.buffer = buffer;
            self.redraw = true;
        }
        Ok(())
    }
    pub fn resize(&mut self, size: Size) {
        self.size = size;
        self.redraw = true;
    }
    fn move_text_location(&mut self, direction: &Direction) {
        let Location {
            mut grapheme_index,
            mut line_index,
        } = self.text_location;
        let Size { height, .. } = self.size;
        match direction {
            Direction::Up => {
                line_index = line_index.saturating_sub(1);
                if let Some(line) = self.buffer.lines.get(line_index) {
                    grapheme_index = min(grapheme_index, line.grapheme_count().saturating_sub(1));
                }
            }
            Direction::Down => {
                if line_index.saturating_add(1) >= self.buffer.height() {
                    grapheme_index = 0;
                    line_index = line_index.saturating_add(1);
                } else {
                    line_index = line_index.saturating_add(1);
                    if let Some(line) = self.buffer.lines.get(line_index) {
                        grapheme_index =
                            min(grapheme_index, line.grapheme_count().saturating_sub(1));
                    }
                }
            }
            Direction::Left => {
                if grapheme_index == 0 {
                    if let Some(line) = self.buffer.lines.get(line_index - 1) {
                        line_index = line_index.saturating_sub(1);
                        grapheme_index = line.grapheme_count().saturating_sub(1);
                    }
                } else {
                    grapheme_index = grapheme_index.saturating_sub(1);
                }
            }
            Direction::Right => {
                if let Some(line) = self.buffer.lines.get(line_index) {
                    if grapheme_index >= line.grapheme_count() {
                        line_index = line_index.saturating_add(1);
                        grapheme_index = 0;
                    } else {
                        grapheme_index =
                            min(grapheme_index.saturating_add(1), line.grapheme_count());
                    }
                } else {
                    grapheme_index = 0;
                }
            }
            Direction::PageUp => line_index = line_index.saturating_sub(height).saturating_sub(1),
            Direction::PageDown => line_index = line_index.saturating_add(height).saturating_sub(1),
            Direction::Home => grapheme_index = 0,
            Direction::End => {
                grapheme_index = self
                    .buffer
                    .lines
                    .get(line_index)
                    .map_or(0, |line| line.width_until(line.grapheme_count()))
            }
        }
        line_index = min(line_index, self.buffer.height());
        self.text_location = Location {
            line_index,
            grapheme_index,
        };
        self.scroll_location_into_view();
    }
    pub fn handle_command(&mut self, command: EditorCommand) {
        match command {
            EditorCommand::Quit => {}
            EditorCommand::Move(direction) => self.move_text_location(&direction),
            EditorCommand::Resize(size) => self.resize(size),
        }
    }
    fn text_location_to_position(&self) -> Position {
        let row = self.text_location.line_index;
        let col = self.buffer.lines.get(row).map_or(0, |line| {
            line.width_until(self.text_location.grapheme_index)
        });
        Position { row, col }
    }
    pub fn caret_position(&self) -> Position {
        self.text_location_to_position()
            .saturating_sub(self.scroll_offset)
    }
    fn scroll_location_into_view(&mut self) {
        let Position { col, row } = self.text_location_to_position();
        let Size { width, height } = self.size;
        let mut offset_changed = false;

        // Scroll vertically
        if row < self.scroll_offset.row {
            self.scroll_offset.row = row;
            offset_changed = true;
        } else if row >= self.scroll_offset.row.saturating_add(height) {
            self.scroll_offset.row = row.saturating_sub(height).saturating_add(1);
            offset_changed = true;
        }

        //Scroll horizontally
        if col < self.scroll_offset.col {
            self.scroll_offset.col = col;
            offset_changed = true;
        } else if col >= self.scroll_offset.col.saturating_add(width) {
            self.scroll_offset.col = col.saturating_sub(width).saturating_add(1);
            offset_changed = true;
        }
        self.redraw = offset_changed;
    }
}
