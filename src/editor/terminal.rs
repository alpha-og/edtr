use crossterm::cursor::{Hide, MoveTo, Show};
use crossterm::style::Print;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, size, Clear, ClearType};
use crossterm::{queue, Command};

use std::io::{stdout, Write};

pub struct Terminal;
pub struct Size {
    pub width: usize,
    pub height: usize,
}
#[derive(Copy, Clone)]
pub struct Position {
    pub col: usize,
    pub row: usize,
}

impl Terminal {
    pub fn initialize() -> Result<(), std::io::Error> {
        Self::clear_screen()?;
        enable_raw_mode()
    }
    pub fn terminate() -> Result<(), std::io::Error> {
        disable_raw_mode()?;
        Self::clear_screen()?;
        Self::print(String::from("Bye!\r\n"))
    }
    pub fn flush_buffer() -> Result<(), std::io::Error> {
        stdout().flush()
    }
    pub fn size() -> Result<Size, std::io::Error> {
        let size = size()?;
        let width = size.0 as usize;
        let height = size.1 as usize;
        Ok(Size { width, height })
    }
    pub fn print(str: String) -> Result<(), std::io::Error> {
        Self::queue_command(Print(str))?;
        Self::flush_buffer()
    }
    pub fn clear_screen() -> Result<(), std::io::Error> {
        Self::queue_command(Clear(ClearType::All))?;
        Self::flush_buffer()?;
        Self::move_caret_to(Position { col: 0, row: 0 })
    }
    pub fn clear_line() -> Result<(), std::io::Error> {
        Self::queue_command(Clear(ClearType::CurrentLine))?;
        Self::flush_buffer()
    }
    pub fn move_caret_to(position: Position) -> Result<(), std::io::Error> {
        Self::queue_command(MoveTo(position.col as u16, position.row as u16))?;
        Self::flush_buffer()
    }
    fn queue_command(command: impl Command) -> Result<(), std::io::Error> {
        queue!(stdout(), command)
    }
    pub fn hide_caret() -> Result<(), std::io::Error> {
        Self::queue_command(Hide)?;
        Self::flush_buffer()?;
        Ok(())
    }
    pub fn show_caret() -> Result<(), std::io::Error> {
        Self::queue_command(Show)?;
        Self::flush_buffer()?;
        Ok(())
    }
}
