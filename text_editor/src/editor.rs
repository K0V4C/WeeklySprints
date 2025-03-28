mod editor_command;
mod status_bar;
mod terminal;
mod view;

use crossterm::event::Event;
use status_bar::StatusBar;
use std::io::Error;

use crossterm::event::read;

use editor_command::EditorCommand;
use terminal::{CaretPosition, Terminal, TerminalSize};
use view::View;

#[derive(Default)]
pub struct DocumentStatus {
    caret_position: CaretPosition,
    file_name: Option<String>,
    number_of_lines: usize,
    is_modified: bool,
}

pub struct Editor {
    should_quit: bool,
    view: View,
    status_bar: StatusBar,
}

impl Editor {
    pub fn new() -> Result<Self, Error> {
        let current_hook = std::panic::take_hook();

        std::panic::set_hook(Box::new(move |panic_info| {
            let _ = Terminal::terminate();
            current_hook(panic_info);
        }));

        Terminal::init()?;

        let status_bar_height = 1;
        let status_bar = StatusBar::new(status_bar_height);

        let mut view = View::new(status_bar.height() + 1);
        Self::load_file(&mut view);

        Ok(Editor {
            should_quit: false,
            view,
            status_bar,
        })
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
            view.load(name);
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
                }
                self.view.handle_command(command);
            }
        }
    }

    fn refresh_screen(&mut self) {
        let _ = Terminal::hide_caret();
        self.view.render();
        self.status_bar.render();
        let _ = Terminal::move_caret_to(self.view.caret_position());
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
