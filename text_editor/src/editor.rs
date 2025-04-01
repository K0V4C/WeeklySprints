use crate::editor::ui_component::UiComponent;

mod caret_position;
mod command;
mod document_status;
mod line;
mod terminal;
mod ui_component;

use crate::editor::Command::{Edit, Move, System};
use crate::editor::command::System::{Abort, Quit, Resize, Save, Search};
use caret_position::CaretPosition;
use command::Command;
use crossterm::event::Event;
use std::io::Error;
use ui_component::command_bar::CommandBar;
use ui_component::{
    message_bar::{FIVE_SECONDS, MessageBar},
    status_bar::StatusBar,
    view::View,
};

use crossterm::event::read;

use terminal::{Terminal, TerminalSize};

const QUIT_COUNTER_START: usize = 3;

enum Mode {
    Editing,
    SavingAs,
    Searching,
}

const SAVE_PROMPT: &str = "Save As: ";
const SEARCH_PROMPT: &str = "Search (Esc to cancel, Arrows to navigate): ";

pub struct Editor {
    should_quit: bool,
    view: View,
    status_bar: StatusBar,
    message_bar: MessageBar,
    command_bar: CommandBar,
    quit_counter: usize,
    mode: Mode,
}

impl Editor {
    pub fn new() -> Result<Self, Error> {
        let current_hook = std::panic::take_hook();

        std::panic::set_hook(Box::new(move |panic_info| {
            let _ = Terminal::terminate();
            current_hook(panic_info);
        }));

        Terminal::init()?;

        let (mut view, status_bar, message_bar, command_bar) = Self::create_components();

        Self::load_file(&mut view);

        Ok(Editor {
            should_quit: false,
            view,
            status_bar,
            message_bar,
            command_bar,
            quit_counter: QUIT_COUNTER_START,
            mode: Mode::Editing,
        })
    }

    fn create_components() -> (View, StatusBar, MessageBar, CommandBar) {
        let message_bar = MessageBar::new();
        let status_bar = StatusBar::new();
        let command_bar = CommandBar::new();

        let view = View::new(2);

        (view, status_bar, message_bar, command_bar)
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
        match self.mode {
            Mode::Editing => self.handle_editing_command(command),
            Mode::SavingAs => self.handle_save_command(command),
            Mode::Searching => self.handle_search_command(command),
        }
    }

    fn handle_editing_command(&mut self, command: Command) {
        match command {
            System(Quit) => self.quit_try(),
            System(Resize(size)) => self.resize(size),

            _ => self.reset_quit_counter(),
        }

        match command {
            System(Quit | Resize(_) | Abort) => (),
            System(Search) => self.handle_search(),
            System(Save) => self.handle_save(),
            Move(move_command) => self.view.handle_move_command(move_command),
            Edit(edit_command) => self.view.handle_edit_command(edit_command),
        }
    }

    fn handle_save_command(&mut self, command: Command) {
        match command {
            System(Resize(size)) => self.resize(size),
            System(Abort) => self.exit_mode(),
            Edit(edit_command) => self.command_bar.handle_edit_command(edit_command),
            _ => (),
        }

        if let Command::Edit(command::Edit::Enter) = command {
            self.view
                .set_buffer_file(&self.command_bar.get_command_line());
            self.exit_mode();
            self.handle_save();
        }
    }

    fn handle_search_command(&mut self, command: Command) {
        match command {
            System(Resize(size)) => self.resize(size),
            System(Abort) => self.dismiss_search(),
            Edit(command::Edit::Enter) => self.exit_search(),
            Edit(edit_command) => self.handle_edit_search(edit_command),
            Move(move_command) => self.handle_move_search(move_command),
            System(_) => (),
        }
    }

    fn handle_move_search(&mut self, move_command: command::Move) {
        let search_string = self.command_bar.get_line();

        match move_command {
            command::Move::Right | command::Move::Down => self.view.search_next(&search_string),
            command::Move::Left | command::Move::Up => self.view.search_previous(&search_string),
            _ => (),
        }
    }

    fn handle_edit_search(&mut self, edit_command: command::Edit) {
        self.command_bar.handle_edit_command(edit_command);
        let search_string = self.command_bar.get_line();
        self.view.search(&search_string);
    }

    fn dismiss_search(&mut self) {
        self.view.dissmiss_search();
        self.exit_mode();
    }

    fn exit_search(&mut self) {
        self.view.exit_search();
        self.exit_mode();
        #[cfg(debug_assertions)]
        {
            debug_assert!(matches!(self.mode, Mode::Editing));
        }
    }

    // ================================================== Mode switching ================================================================

    fn handle_search(&mut self) {
        self.view.enter_search();
        self.enter_search_mode();
    }

    fn handle_save(&mut self) {
        if !self.view.is_file_given() {
            self.enter_save_mode();
            return;
        }

        if matches!(self.mode, Mode::Editing) {
            if self.view.handle_save().is_err() {
                self.message_bar.update_message("Error saving the file");
            } else {
                self.message_bar.update_message("File saved sucessfully!");
            }
        }
    }

    // ==================================================== Mode manipulation ========================================================

    fn enter_save_mode(&mut self) {
        self.mode = Mode::SavingAs;
        self.command_bar.set_prompt(SAVE_PROMPT.to_string());
    }

    fn enter_search_mode(&mut self) {
        self.mode = Mode::Searching;
        self.command_bar.set_prompt(SEARCH_PROMPT.to_string());
    }

    fn exit_mode(&mut self) {
        self.mode = Mode::Editing;
        self.command_bar.clear_line();
        self.message_bar.back_to_defaulf();
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

        self.command_bar.resize(TerminalSize {
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
        self.move_caret();
        let _ = Terminal::show_caret();
        let _ = Terminal::draw();
    }

    fn move_caret(&self) {
        let position =
            if (matches!(self.mode, Mode::Editing) || matches!(self.mode, Mode::Searching)) {
                self.view.caret_position()
            } else {
                let row = Terminal::size().unwrap_or_default().rows.saturating_sub(1);
                let caret_pos = self.command_bar.caret_position_column();

                CaretPosition {
                    row,
                    column: caret_pos,
                }
            };
        let _ = Terminal::move_caret_to(position);
    }

    fn render_components(&mut self) {
        let terminal_size = Terminal::size().unwrap_or_default();

        if terminal_size.columns == 0 || terminal_size.rows == 0 {
            return;
        }

        // Order of rendering here is important

        if terminal_size.rows > 2 {
            self.view.render(0);
        }

        if terminal_size.rows > 1 {
            self.status_bar.render(terminal_size.rows.saturating_sub(2));
        }

        if matches!(self.mode, Mode::Editing) {
            self.message_bar.check_message_expired(FIVE_SECONDS);
            self.message_bar
                .render(terminal_size.rows.saturating_sub(1));
        } else {
            self.command_bar
                .render(terminal_size.rows.saturating_sub(1));
        }
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
