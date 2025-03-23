mod terminal;
mod editor_command;
mod view;

use std::io::Error;
use crossterm::event::Event;

use crossterm::event::{
    KeyEvent, KeyEventKind, read,
};

use editor_command::EditorCommand;
use terminal::Terminal;
use view::View;

pub struct Editor {
    should_quit: bool,
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
            view
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

        let should_process = match &event {
            Event::Key(KeyEvent { kind, ..}) => kind == &KeyEventKind::Press,
            Event::Resize(_,_) => true,
            _ => false
        };

        // If there is no work to be done dont go into details
        #[cfg(debug_assertions)]
        {
            assert!(should_process, "Recieved and discarded unsuported non-press event");
        }

        match EditorCommand::try_from(event) {
            Ok(command) => {
                if matches!(command, EditorCommand::Quit) {
                    self.should_quit = true;
                } else {
                    self.view.handle_command(command);
                }
            },
            Err(err) => {
                #[cfg(debug_assertions)]
                {
                    panic!("Could not handle commmand: {err}");
                }
            }
        }
    }

    fn refresh_screen(&mut self) {
        let _ = Terminal::hide_caret();
        self.view.render();
        let _ = Terminal::move_caret_to(self.view.get_position());
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
