mod terminal;
mod view;

use std::{cmp::min, io::Error};

use crossterm::event::{
    Event::{self, Key, Resize},
    KeyCode::{self, Char, Down, End, Home, Left, PageDown, PageUp, Right, Up},
    KeyEvent, KeyEventKind, KeyModifiers, read,
};

use terminal::{CaretPosition, Terminal, TerminalSize};
use view::View;

#[derive(Clone, Copy, Default)]
struct Location {
    x: usize,
    y: usize,
}

pub struct Editor {
    should_quit: bool,
    location: Location,
    view: View,
}

impl Editor {

    pub fn new() -> Result<Self, Error> {

        let current_hook = std::panic::take_hook();

        std::panic::set_hook(Box::new( move |panic_info| {
            let _ = Terminal::terminate();
            current_hook(panic_info);
        }));

        Terminal::init()?;

        let mut view = View::default();
        Self::load_file(&mut view);

        Ok(Editor{
            should_quit: false,
            location: Location::default(),
            view
        })
    }

    pub fn run(&mut self) {

        self.location = Location { x: 0, y: 0 };

        loop {
            self.refresh_screen();
            if self.should_quit {
                break;
            }
            match read() {
                Ok(event) => {
                    self.evaluate_event(&event);
                }

                Err(error) => {
                    #[cfg(debug_assertions)]
                    {
                        panic!("Could no read event: {error:?}");
                    }
                }
            }
        }

    }

    fn load_file(view: &mut View) {
        let env_vars: Vec<String> = std::env::args().collect();

        if env_vars.len() == 1 {
            return;
        }

        if let Some(name) = env_vars.get(1) {
            view.load(name);
        }

        // For now i will ignore everything past the file name
    }

    fn move_point(&mut self, code: KeyCode) {
        let TerminalSize { columns, rows } = Terminal::size().unwrap_or_default();
        let Location { mut x, mut y } = self.location;

        match code {
            Left => {
                x = x.saturating_sub(1);
            }

            Right => {
                x = min(x.saturating_add(1), columns.saturating_sub(1));
            }

            Up => {
                y = y.saturating_sub(1);
            }

            Down => {
                y = min(y.saturating_add(1), rows.saturating_sub(1));
            }

            Home => {
                x = 0;
            }

            End => {
                x = columns.saturating_sub(1);
            }

            PageUp => {
                y = 0;
            }

            PageDown => {
                y = rows.saturating_sub(1);
            }

            _ => (),
        }

        self.location = Location { x, y };
    }

    fn evaluate_event(&mut self, event: &Event) {
        match event {
            Key(KeyEvent {
                code,
                modifiers,
                kind: KeyEventKind::Press, // This is so it works on windows
                ..
            }) => match code {
                Char('q') if *modifiers == KeyModifiers::CONTROL => {
                    self.should_quit = true;
                }

                Left | Right | Up | Down | Home | End | PageUp | PageDown => {
                    self.move_point(*code);
                }

                _ => (),
            },

            Resize(columns, rows) => {
                let columns = *columns as usize;
                let rows = *rows as usize;
                self.view.resize(TerminalSize { columns, rows });
            }

            _ => (),
        }
    }

    fn refresh_screen(&mut self) {
        let _ = Terminal::hide_caret();
        if self.should_quit {
            let _ = Terminal::clear_screen();
            println!("Chao!");
        } else {
            self.view.render();
            let _ = Terminal::move_caret_to(CaretPosition {
                column: self.location.x,
                row: self.location.y,
            });
        }
        let _ = Terminal::show_caret();
        let _ = Terminal::draw();
    }
}

impl Drop for Editor {
    fn drop(&mut self) {
        let _ = Terminal::terminate();
        if self.should_quit {
            let _ = Terminal::print("Chaos!\r\n");
        }
    }
}
