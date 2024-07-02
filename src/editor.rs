mod terminal;
use terminal::{Position, Size, Terminal};

mod view;
use view::View;

use crossterm::event::{
    read,
    Event::{self, Key, Resize},
    KeyCode, KeyEvent, KeyEventKind, KeyModifiers,
};

#[derive(Default)]
pub struct Editor {
    should_quit: bool,
    view: View,
}

impl Editor {
    pub fn new() -> Result<Self, std::io::Error> {
        let current_hook = std::panic::take_hook();
        std::panic::set_hook(Box::new(move |panic_info| {
            let _ = Terminal::terminate();
            current_hook(panic_info);
        }));
        Terminal::initialize()?;
        let mut view = View::default();

        let args: Vec<String> = std::env::args().collect();
        if let Some(arg) = args.get(1) {
            view.load(&arg.to_string())?;
        }
        Ok(Self {
            view,
            should_quit: false,
        })
    }

    pub fn run(&mut self) {
        loop {
            self.refresh_screen();
            if self.should_quit {
                break;
            }

            match read() {
                Ok(event) => self.evaluate_event(event),
                Err(err) => {
                    #[cfg(debug_assertions)]
                    {
                        panic!("Unable to read event {err:?}")
                    }
                }
            }
        }
    }
    fn evaluate_event(&mut self, event: Event) {
        match event {
            Key(KeyEvent {
                code,
                modifiers,
                kind: KeyEventKind::Press,
                ..
            }) => match code {
                KeyCode::Char('q') => {
                    if KeyModifiers::CONTROL == modifiers {
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
                    self.view.move_point(code);
                }
                _ => {}
            },
            Resize(width, height) => {
                let width = width as usize;
                let height = height as usize;
                self.view.resize(Size { width, height });
            }
            _ => {}
        }
    }
    fn refresh_screen(&mut self) {
        let _ = Terminal::hide_caret();
        let _ = Terminal::move_caret_to(Default::default());
        if self.should_quit {
        } else {
            let _ = self.view.render();
            let _ = Terminal::move_caret_to(Position {
                col: self.view.location.x,
                row: self.view.location.y,
            });
        }
        let _ = Terminal::show_caret();
        let _ = Terminal::flush_buffer();
    }
}

impl Drop for Editor {
    fn drop(&mut self) {
        let _ = Terminal::terminate();
        let _ = Terminal::clear_screen();
        let _ = Terminal::print("Bye!\r\n");
        let _ = Terminal::flush_buffer();
    }
}
