mod terminal;
use terminal::Terminal;

use crossterm::event::{read, Event, Event::Key, KeyCode::Char, KeyEvent, KeyModifiers};

pub struct Editor {
    should_quit: bool,
}

impl Editor {
    pub fn default() -> Self {
        Editor { should_quit: false }
    }

    fn repl(&mut self) -> Result<(), std::io::Error> {
        loop {
            self.refresh_screen()?;
            if self.should_quit {
                break;
            }

            let event = read()?;
            self.evaluate_event(&event);
        }
        Ok(())
    }

    fn evaluate_event(&mut self, event: &Event) {
        if let Key(KeyEvent {
            code, modifiers, ..
        }) = event
        {
            if let Char('q') = code {
                if KeyModifiers::CONTROL == *modifiers {
                    self.should_quit = true;
                };
            }
        }
    }

    fn refresh_screen(&self) -> Result<(), std::io::Error> {
        if self.should_quit {
            Terminal::clear_screen()?;
            println!("Bye!\r\n");
        } else {
            Self::draw_rows()?;
        }
        Ok(())
    }

    fn draw_rows() -> Result<(), std::io::Error> {
        let (_columns, rows) = Terminal::size()?;
        for row in 0..rows {
            Terminal::move_cursor_to(0, row)?;
            print!("~");
            if row + 1 < rows {
                print!("\r\n");
            }
        }
        Ok(())
    }

    pub fn run(&mut self) {
        Terminal::initialize().unwrap();
        let result = self.repl();
        Terminal::terminate().unwrap();
        result.unwrap();
    }
}
