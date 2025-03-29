use crate::editor::terminal::{Terminal, TerminalSize};

use super::UiComponent;

#[derive(Default)]
pub struct MessageBar {
    needs_redraw: bool,
    size: TerminalSize,
    message_string: String,
}

impl MessageBar {
    // ======================================== PUBLIC INTERFACE ==================================================
    pub fn new() -> Self {
        let size = Terminal::size().unwrap_or_default();

        MessageBar {
            needs_redraw: true,
            size,
            message_string: Self::default_message(),
        }
    }

    pub fn resize(&mut self, new_size: TerminalSize) {
        self.size = new_size;
        self.needs_redraw = true;
    }

    // ======================================== HELPER METHODS =======================================================

    fn default_message() -> String {
        "HELP: Ctrl-S = save | Ctrl-Q = quit".to_string()
    }
}

impl UiComponent for MessageBar {
    /// Marks if ui component need to be redrawn
    fn mark_redraw(&mut self, needs_redraw: bool) {
        self.needs_redraw = needs_redraw;
    }

    /// Get status of redraw
    fn needs_redraw(&self) -> bool {
        self.needs_redraw
    }

    /// Set the size of the component
    fn set_size(&mut self, new_size: TerminalSize) {
        self.size = new_size;
    }

    /// Method to actually draw the component, must be implemented by each component
    fn draw(&mut self, origin_y: usize) -> Result<(), std::io::Error> {
        Terminal::print_row(origin_y, &self.message_string)
    }
}
