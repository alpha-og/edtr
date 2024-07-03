mod buffer;
mod line;
mod location;

use std::cmp::min;

use buffer::Buffer;
use line::Line;
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
    location: Location,
    scroll_offset: Location,
}

impl Default for View {
    fn default() -> Self {
        Self {
            buffer: Default::default(),
            redraw: true,
            size: Terminal::size().unwrap_or_default(),
            location: Default::default(),
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
        let top = self.scroll_offset.y;
        for row in 0..height {
            if let Some(line) = self.buffer.lines.get(row.saturating_add(top)) {
                let left = self.scroll_offset.x;
                let right = self.scroll_offset.x.saturating_add(width);
                Self::render_line(row, &line.get(left..right));
            } else if self.buffer.is_empty() && row == height / 3 {
                Self::render_line(row, &Self::generate_welcome_message(width));
            } else {
                Self::render_line(row, "~");
            }
        }
        self.redraw = false;
    }
    fn render_line(row: usize, content: &str) {
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
        let Location { mut x, mut y } = self.location;
        let Size { height, .. } = self.size;
        match direction {
            Direction::Up => {
                y = y.saturating_sub(1);
                if let Some(line) = self.buffer.lines.get(y) {
                    x = min(x, line.len().saturating_sub(1));
                }
            }
            Direction::Down => {
                if y.saturating_add(1) >= self.buffer.lines.len() {
                    x = 0;
                    y = y.saturating_add(1);
                } else {
                    y = y.saturating_add(1);
                    if let Some(line) = self.buffer.lines.get(y) {
                        x = min(x, line.len().saturating_sub(1));
                    }
                }
            }
            Direction::Left => {
                if x == 0 {
                    if let Some(line) = self.buffer.lines.get(y - 1) {
                        y = y.saturating_sub(1);
                        x = line.len().saturating_sub(1);
                    }
                } else {
                    x = x.saturating_sub(1);
                }
            }
            Direction::Right => {
                if let Some(line) = self.buffer.lines.get(y) {
                    if x >= line.len() {
                        y = y.saturating_add(1);
                        x = 0;
                    } else {
                        x = min(x.saturating_add(1), line.len());
                    }
                } else {
                    x = 0;
                }
            }
            Direction::PageUp => y = y.saturating_sub(height).saturating_sub(1),
            Direction::PageDown => y = y.saturating_add(height).saturating_sub(1),
            Direction::Home => x = 0,
            Direction::End => x = self.buffer.lines.get(y).map_or(0, Line::len),
        }
        y = min(y, self.buffer.lines.len());
        self.location = Location { x, y };
        self.scroll_location_into_view();
    }
    pub fn handle_command(&mut self, command: EditorCommand) {
        match command {
            EditorCommand::Quit => {}
            EditorCommand::Move(direction) => self.move_text_location(&direction),
            EditorCommand::Resize(size) => self.resize(size),
        }
    }
    pub fn get_position(&self) -> Position {
        self.location.subtract(&self.scroll_offset).into()
    }
    fn scroll_location_into_view(&mut self) {
        let Location { x, y } = self.location;
        let Size { width, height } = self.size;
        let mut offset_changed = false;

        // Scroll vertically
        if y < self.scroll_offset.y {
            self.scroll_offset.y = y;
            offset_changed = true;
        } else if y >= self.scroll_offset.y.saturating_add(height) {
            self.scroll_offset.y = y.saturating_sub(height).saturating_add(1);
            offset_changed = true;
        }

        //Scroll horizontally
        if x < self.scroll_offset.x {
            self.scroll_offset.x = x;
            offset_changed = true;
        } else if x >= self.scroll_offset.x.saturating_add(width) {
            self.scroll_offset.x = x.saturating_sub(width).saturating_add(1);
            offset_changed = true;
        }
        self.redraw = offset_changed;
    }
}
