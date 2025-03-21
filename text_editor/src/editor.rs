mod terminal;

use std::cmp::min;

use crossterm::event::{
    Event::{self, Key},
    KeyCode::{self, Char, Down, End, Home, Left, PageDown, PageUp, Right, Up},
    KeyEvent, KeyEventKind, KeyModifiers, read,
};

use terminal::{CaretPosition, Terminal, TerminalSize};

const EDITOR_NAME: &str = "HECTO";
const EDITOR_VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Clone, Copy, Default)]
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
    pub fn run(&mut self) {
        Terminal::init().unwrap();
        let result = self.repl();
        Terminal::terminate().unwrap();
        result.unwrap();
    }

    fn repl(&mut self) -> Result<(), std::io::Error> {
        Self::draw_rows()?;
        Self::draw_welcome_message()?;
        Self::caret_to_tilde()?;
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
        if let Key(KeyEvent {
            code,
            modifiers,
            kind: KeyEventKind::Press, // This is so it works on windows
            ..
        }) = event
        {
            match code {
                Char('q') if *modifiers == KeyModifiers::CONTROL => {
                    self.should_quit = true;
                }

                Left | Right | Up | Down | Home | End | PageUp | PageDown => {
                    self.move_point(*code)?;
                }

                _ => (),
            }
        }
        Ok(())
    }
    fn refresh_screen(&self) -> Result<(), std::io::Error> {
        Terminal::hide_caret()?;
        if self.should_quit {
            Terminal::clear_screen()?;
            println!("Chao!");
        } else {
            Terminal::move_caret_to(CaretPosition {
                column: self.location.x,
                row: self.location.y,
            })?;
        }
        Terminal::show_caret()?;
        Terminal::draw()?;
        Ok(())
    }

    fn caret_to_start() -> Result<(), std::io::Error> {
        Terminal::move_caret_to(CaretPosition { column: 0, row: 0 })
    }

    fn caret_to_tilde() -> Result<(), std::io::Error> {
        Terminal::move_caret_to(CaretPosition { column: 1, row: 0 })
    }

    fn draw_rows() -> Result<(), std::io::Error> {
        Self::caret_to_start()?;
        for _ in 0..Terminal::size()?.rows {
            Terminal::clear_line()?;
            Terminal::print("~\r\n")?;
        }
        Terminal::print("~")?;
        Self::caret_to_start()?;

        Ok(())
    }

    fn draw_welcome_message() -> Result<(), std::io::Error> {
        let size = Terminal::size()?;
        let start_point = size.rows / 3;
        let length_of_terminal = size.columns;

        let blank_line =
            String::from("|") + " ".repeat(length_of_terminal.saturating_sub(2)).as_str() + "|";

        // Draw top bar
        Terminal::move_caret_to(CaretPosition {
            column: 0,
            row: start_point,
        })?;
        Terminal::print("=".repeat(length_of_terminal).as_str())?;

        // Draw sides
        for _ in 0..5 {
            Terminal::print(blank_line.as_str())?;
        }

        // Draw Name and version number
        let half_width =
            ((length_of_terminal - EDITOR_NAME.len() - EDITOR_VERSION.len()).saturating_sub(3)) / 2;
        let blank_space = " ".repeat(half_width);

        let center_line = format!("|{blank_space}{EDITOR_NAME} {EDITOR_VERSION}{blank_space}|");

        Terminal::print(center_line.as_str())?;

        // Draw sides
        for _ in 0..5 {
            Terminal::print(blank_line.as_str())?;
        }

        // Draw bottom bar
        Terminal::print("=".repeat(length_of_terminal).as_str())?;

        Ok(())
    }
}
