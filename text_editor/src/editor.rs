mod terminal;
mod view;

use std::cmp::min;

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

#[derive(Default)]
pub struct Editor {
    should_quit: bool,
    location: Location,
    view: View,
}

impl Editor {
    pub fn run(&mut self) {
        Terminal::init().unwrap();

        // Check for env vars and if files passed load it into view
        self.load_file().unwrap();

        let result = self.repl();
        Terminal::terminate().unwrap();
        result.unwrap();
    }

    fn repl(&mut self) -> Result<(), std::io::Error> {
        self.location = Location { x: 0, y: 0 };

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

    fn load_file(&mut self) -> Result<(), std::io::Error> {
        let env_vars: Vec<String> = std::env::args().collect();

        if env_vars.len() == 1 {
            return Ok(());
        }

        if let Some(name) = env_vars.get(1) {
            self.view.load(name)?;
        }

        // For now i will ignore everything past the file name

        Ok(())
    }

    fn move_point(&mut self, code: KeyCode) -> Result<(), std::io::Error> {
        let TerminalSize { columns, rows } = Terminal::size()?;
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

        Ok(())
    }

    fn evaluate_event(&mut self, event: &Event) -> Result<(), std::io::Error> {
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
                    self.move_point(*code)?;
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

        Ok(())
    }
    fn refresh_screen(&mut self) -> Result<(), std::io::Error> {
        Terminal::hide_caret()?;
        if self.should_quit {
            Terminal::clear_screen()?;
            println!("Chao!");
        } else {
            self.view.render()?;
            Terminal::move_caret_to(CaretPosition {
                column: self.location.x,
                row: self.location.y,
            })?;
        }
        Terminal::show_caret()?;
        Terminal::draw()?;
        Ok(())
    }
}
