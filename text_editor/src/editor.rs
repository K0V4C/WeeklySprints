mod editor_command;
mod document_status;
mod message_bar;
mod status_bar;
mod terminal;
mod view;

use crossterm::event::Event;
use message_bar::MessageBar;
use status_bar::StatusBar;
use std::io::Error;

use crossterm::event::read;

use editor_command::EditorCommand;
use terminal::{Terminal, TerminalSize};
use view::View;

pub struct Editor {
    should_quit: bool,
    view: View,
    status_bar: StatusBar,
    message_bar: MessageBar
}

impl Editor {
    pub fn new() -> Result<Self, Error> {
        let current_hook = std::panic::take_hook();

        std::panic::set_hook(Box::new(move |panic_info| {
            let _ = Terminal::terminate();
            current_hook(panic_info);
        }));

        Terminal::init()?;

        let (mut view, status_bar, message_bar) = Self::create_components();

        Self::load_file(&mut view);

        Ok(Editor {
            should_quit: false,
            view,
            status_bar,
            message_bar
        })
    }

    fn create_components() -> (View, StatusBar, MessageBar) {

        let message_bar_height = 1;
        let message_bar = MessageBar::new(message_bar_height);

        let status_bar_height = 1;
        let status_bar = StatusBar::new(status_bar_height);

        let view = View::new(status_bar.height() + message_bar.height());

        (view, status_bar, message_bar)
    }

    pub fn run(&mut self) {
        loop {

            let status = self.view.get_status();
            self.status_bar.update_status(status);

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
        }
    }

    fn load_file(view: &mut View) {
        let env_vars: Vec<String> = std::env::args().collect();

        if env_vars.len() == 1 {
            return;
        }

        if let Some(name) = env_vars.get(1) {
            let _ = Terminal::set_title(&name);
            view.load(name);
        } else {
            let _ = Terminal::set_title("Hecto");
        }

        // For now i will ignore everything past the file name
    }

    fn evaluate_event(&mut self, event: Event) {
        if let Ok(command) = EditorCommand::try_from(event) {
            if matches!(command, EditorCommand::Quit) {
                self.should_quit = true;
            } else {
                if let EditorCommand::Resize(ts) = command {
                    let new_size = TerminalSize {
                        rows: 1,
                        columns: ts.columns,
                    };
                    self.status_bar.resize(new_size);
                    self.message_bar.resize(new_size);
                }
                self.view.handle_command(command);
            }
        }
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

        match terminal_size.rows {

            _ if terminal_size.rows == 1 => {
                self.message_bar.render();
            }

            _ if terminal_size.rows < 3 => {
                self.status_bar.render();
                self.message_bar.render();
            }

            _ if terminal_size.rows > 3 => {
                self.view.render();
                self.status_bar.render();
                self.message_bar.render();
            }

            _ => {
                panic!("BOOM");
            }
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
