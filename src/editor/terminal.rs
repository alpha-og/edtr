use crossterm::cursor::{Hide, MoveTo, Show};
use crossterm::style::Print;
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, size, Clear, ClearType, EnterAlternateScreen,
    LeaveAlternateScreen,
};
use crossterm::{queue, Command};

use std::io::{stdout, Write};

pub struct Terminal;
#[derive(Default, Copy, Clone)]
pub struct Size {
    pub width: usize,
    pub height: usize,
}
#[derive(Default, Copy, Clone)]
pub struct Position {
    pub col: usize,
    pub row: usize,
}

impl Terminal {
    pub fn initialize() -> Result<(), std::io::Error> {
        Self::enter_alternate_screen()?;
        Self::clear_screen()?;
        Self::flush_buffer()?;
        enable_raw_mode()?;
        Ok(())
    }
    pub fn terminate() -> Result<(), std::io::Error> {
        Self::leave_alternate_screen()?;
        Self::show_caret()?;
        Self::flush_buffer()?;
        disable_raw_mode()?;
        Ok(())
    }
    pub fn flush_buffer() -> Result<(), std::io::Error> {
        stdout().flush()?;
        Ok(())
    }
    pub fn size() -> Result<Size, std::io::Error> {
        let size = size()?;
        let width = size.0 as usize;
        let height = size.1 as usize;
        Ok(Size { width, height })
    }
    pub fn print(str: &str) -> Result<(), std::io::Error> {
        Self::queue_command(Print(str))
    }
    pub fn clear_screen() -> Result<(), std::io::Error> {
        Self::queue_command(Clear(ClearType::All))?;
        Self::move_caret_to(Default::default())
    }
    pub fn clear_line() -> Result<(), std::io::Error> {
        Self::queue_command(Clear(ClearType::CurrentLine))
    }
    pub fn move_caret_to(position: Position) -> Result<(), std::io::Error> {
        Self::queue_command(MoveTo(position.col as u16, position.row as u16))
    }
    fn queue_command(command: impl Command) -> Result<(), std::io::Error> {
        queue!(stdout(), command)
    }
    pub fn hide_caret() -> Result<(), std::io::Error> {
        Self::queue_command(Hide)?;
        Ok(())
    }
    pub fn show_caret() -> Result<(), std::io::Error> {
        Self::queue_command(Show)?;
        Ok(())
    }
    pub fn leave_alternate_screen() -> Result<(), std::io::Error> {
        Self::queue_command(LeaveAlternateScreen)?;
        Ok(())
    }
    pub fn enter_alternate_screen() -> Result<(), std::io::Error> {
        Self::queue_command(EnterAlternateScreen)?;
        Ok(())
    }
    pub fn print_row(row: usize, width: usize, content: &str) -> Result<(), std::io::Error> {
        let mut content = String::from(content);
        content.truncate(width);
        Terminal::move_caret_to(Position { col: 0, row })?;
        Terminal::clear_line()?;
        Terminal::print(&content)?;
        Ok(())
    }
}
