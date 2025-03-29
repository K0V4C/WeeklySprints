use crate::editor::ui_component::UiComponent;

mod command;
mod document_status;
mod terminal;
mod ui_component;

use crate::editor::Command::{Edit, Move, System};
use crate::editor::command::System::{Quit, Resize, Save};
use command::Command;
use crossterm::event::Event;
use std::io::Error;
use ui_component::{
    message_bar::{FIVE_SECONDS, MessageBar},
    status_bar::StatusBar,
    view::View,
};

use crossterm::event::read;

use terminal::{Terminal, TerminalSize};

const QUIT_COUNTER_START: usize = 3;

pub struct Editor {
    should_quit: bool,
    view: View,
    status_bar: StatusBar,
    message_bar: MessageBar,
    quit_counter: usize,
}

impl Editor {
    pub fn new() -> Result<Self, Error> {
        let current_hook = std::panic::take_hook();

        std::panic::set_hook(Box::new(move |panic_info| {
            let _ = Terminal::terminate();
            current_hook(panic_info);
        }));

        Terminal::init()?;

        let (mut view, status_bar, mut message_bar) = Self::create_components();

        Self::load_file(&mut view);

        message_bar.update_message("Hi");

        Ok(Editor {
            should_quit: false,
            view,
            status_bar,
            message_bar,
            quit_counter: QUIT_COUNTER_START,
        })
    }

    fn create_components() -> (View, StatusBar, MessageBar) {
        let message_bar = MessageBar::new();
        let status_bar = StatusBar::new();
        let view = View::new(2);

        (view, status_bar, message_bar)
    }

    pub fn run(&mut self) {
        loop {
            self.refresh_screen();

            if self.should_quit {
                break;
            }

            match read() {
                Ok(event) => {
                    self.evaluate_event(event);
                }

                Err(error) => {
                    #[cfg(debug_assertions)]
                    {
                        panic!("Could no read event: {error:?}");
                    }
                }
            }

            let status = self.view.get_status();
            self.status_bar.update_status(status);
        }
    }

    fn load_file(view: &mut View) {
        let env_vars: Vec<String> = std::env::args().collect();

        if env_vars.len() == 1 {
            return;
        }

        if let Some(name) = env_vars.get(1) {
            let _ = Terminal::set_title(name);
            view.load(name);
        } else {
            let _ = Terminal::set_title("Hecto");
        }

        // For now i will ignore everything past the file name
    }

    fn evaluate_event(&mut self, event: Event) {
        if let Ok(command) = Command::try_from(event) {
            self.handle_command(command);
        }
    }

    fn handle_command(&mut self, command: Command) {
        match command {
            System(Quit) => self.quit_try(),
            System(Resize(size)) => self.resize(size),

            _ => self.reset_quit_counter(),
        }

        match command {
            System(Quit | Resize(_)) => (),
            System(Save) => self.handle_save(),
            Move(move_command) => self.view.handle_move_command(move_command),
            Edit(edit_command) => self.view.handle_edit_command(edit_command),
        }
    }

    fn handle_save(&mut self) {
        if self.view.handle_save().is_err() {
            self.message_bar.update_message("Error saving the file");
        } else {
            self.message_bar.update_message("File saved sucessfully!");
        }
    }

    fn reset_quit_counter(&mut self) {
        self.quit_counter = QUIT_COUNTER_START;
    }

    fn quit_try(&mut self) {
        let status = self.view.get_status();
        
        if status.is_modified {
            self.quit_counter = self.quit_counter.saturating_sub(1);
            self.message_bar.update_message(&format!(
                "WARNING! File has unsaved changes. Press Ctrl-Q {} more times to quit.",
                self.quit_counter
            ));
            if self.quit_counter == 0 {
                self.should_quit = true;
            }
        } else {
            self.should_quit = true;
        }
    }

    fn resize(&mut self, new_terminal_size: TerminalSize) {
        self.message_bar.resize(TerminalSize {
            columns: new_terminal_size.columns,
            rows: 1,
        });
        self.status_bar.resize(TerminalSize {
            columns: new_terminal_size.columns,
            rows: 1,
        });
        let new_height = new_terminal_size.rows.saturating_sub(2);
        self.view.resize(TerminalSize {
            columns: new_terminal_size.columns,
            rows: new_height,
        });
    }

    fn refresh_screen(&mut self) {
        let _ = Terminal::hide_caret();
        self.render_components();
        let _ = Terminal::move_caret_to(self.view.caret_position());
        let _ = Terminal::show_caret();
        let _ = Terminal::draw();
    }

    fn render_components(&mut self) {
        let terminal_size = Terminal::size().unwrap_or_default();

        if terminal_size.columns == 0 || terminal_size.rows == 0 {
            return;
        }

        // Order of rendering here is important
        //

        if terminal_size.rows > 2 {
            self.view.render(0);
        }

        if terminal_size.rows > 1 {
            self.status_bar.render(terminal_size.rows.saturating_sub(2));
        }

        self.message_bar.check_message_expired(FIVE_SECONDS);
        self.message_bar
            .render(terminal_size.rows.saturating_sub(1));
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
