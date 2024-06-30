mod terminal;
use std::cmp::min;

use terminal::{Position, Size, Terminal};

use crossterm::event::{
    read,
    Event::{self, Key},
    KeyCode, KeyEvent, KeyEventKind, KeyModifiers,
};

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Copy, Clone, Default)]
struct Location {
    x: usize,
    y: usize,
}
#[derive(Default)]
pub struct Editor {
    should_quit: bool,
    location: Location,
}

impl Editor {
    fn repl(&mut self) -> Result<(), std::io::Error> {
        loop {
            self.refresh_screen()?;
            if self.should_quit {
                break;
            }

            let event = read()?;
            self.evaluate_event(&event)?;
        }
        Ok(())
    }
    fn move_point(&mut self, key_code: KeyCode) -> Result<(), std::io::Error> {
        let Location { mut x, mut y } = self.location;
        let Size { width, height } = Terminal::size()?;
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
        Ok(())
    }

    fn evaluate_event(&mut self, event: &Event) -> Result<(), std::io::Error> {
        if let Key(KeyEvent {
            code,
            modifiers,
            kind: KeyEventKind::Press,
            ..
        }) = event
        {
            match code {
                KeyCode::Char('q') => {
                    if KeyModifiers::CONTROL == *modifiers {
                        self.should_quit = true;
                    }
                }
                KeyCode::Up
                | KeyCode::Down
                | KeyCode::Left
                | KeyCode::Right
                | KeyCode::PageDown
                | KeyCode::PageUp
                | KeyCode::End
                | KeyCode::Home => {
                    self.move_point(*code)?;
                }
                _ => {}
            }
        }
        Ok(())
    }

    fn refresh_screen(&self) -> Result<(), std::io::Error> {
        if !self.should_quit {
            Self::draw_rows(self)?;
        }
        Ok(())
    }

    fn draw_rows(&self) -> Result<(), std::io::Error> {
        Terminal::hide_caret()?;

        let terminal::Size { height, .. } = Terminal::size()?;
        for y in 0..height {
            Self::draw_empty_row(y)?;
            if y == height / 3 {
                Self::draw_wlcm_msg()?;
            }
            if y.saturating_add(1) < height {
                Terminal::print(String::from("\r\n"))?;
            }
        }
        Terminal::move_caret_to(Position {
            col: self.location.x,
            row: self.location.y,
        })?;
        Terminal::show_caret()?;
        Ok(())
    }
    fn draw_empty_row(row: usize) -> Result<(), std::io::Error> {
        Terminal::move_caret_to(Position { col: 0, row })?;
        Terminal::clear_line()?;
        Terminal::print(String::from("~"))
    }
    fn draw_wlcm_msg() -> Result<(), std::io::Error> {
        let terminal::Size { width, height } = Terminal::size()?;
        let mut msg = format!("Welcome to {NAME} -- version {VERSION}");
        msg.truncate(width - 2);
        let length = msg.len();
        let padding_left = (width.saturating_sub(length)) / 2;
        let padding_top = height / 3;
        Terminal::move_caret_to(Position {
            col: padding_left,
            row: padding_top,
        })?;
        Terminal::print(msg)
    }

    pub fn run(&mut self) {
        Terminal::initialize().unwrap();
        let result = self.repl();
        Terminal::terminate().unwrap();
        result.unwrap();
    }
}
