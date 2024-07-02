mod buffer;

use buffer::Buffer;
use crossterm::event::KeyCode;
use std::cmp::min;

use super::terminal::{Size, Terminal};

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Copy, Clone, Default)]
pub struct Location {
    pub x: usize,
    pub y: usize,
}

pub struct View {
    buffer: Buffer,
    redraw: bool,
    size: Size,
    pub location: Location,
    pub scroll_offset: Location,
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
        for row in 0..height {
            if let Some(line) = self.buffer.lines.get(row) {
                self.render_line(row, line);
            } else if self.buffer.is_empty() && row == height / 3 {
                self.render_line(row, &Self::generate_welcome_message(width));
            } else {
                self.render_line(row, "~");
            }
        }
        self.redraw = false;
    }
    fn render_line(&self, row: usize, content: &str) {
        let result = Terminal::print_row(row, self.size.width, content);
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
        }
        self.redraw = true;
        Ok(())
    }
    pub fn resize(&mut self, size: Size) {
        self.size = size;
        self.redraw = true;
    }
    pub fn move_point(&mut self, key_code: KeyCode) {
        let Location { mut x, mut y } = self.location;
        let Size { width, height } = Terminal::size().unwrap_or_default();
        match key_code {
            KeyCode::Up => {
                y = y.saturating_sub(1);
            }
            KeyCode::Down => {
                y = min(height.saturating_sub(1), y.saturating_add(1));
            }
            KeyCode::Left => {
                x = x.saturating_sub(1);
            }
            KeyCode::Right => {
                x = min(width.saturating_sub(1), x.saturating_add(1));
            }
            KeyCode::PageUp => {
                y = 0;
            }
            KeyCode::PageDown => {
                y = height.saturating_sub(1);
            }
            KeyCode::Home => {
                x = 0;
            }
            KeyCode::End => {
                x = width.saturating_sub(1);
            }
            _ => {}
        }
        self.location = Location { x, y };
    }
}
