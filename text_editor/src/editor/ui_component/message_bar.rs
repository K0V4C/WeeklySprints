use std::time::{Duration, Instant};

use crate::editor::terminal::{Terminal, TerminalSize};

use super::UiComponent;

pub const FIVE_SECONDS: Duration = Duration::new(5, 0);

pub struct MessageBar {
    needs_redraw: bool,
    size: TerminalSize,
    message_string: String,
    last_render: Instant,
}

impl MessageBar {
    // ======================================== PUBLIC INTERFACE ==================================================
    pub fn new() -> Self {
        let size = Terminal::size().unwrap_or_default();

        MessageBar {
            needs_redraw: true,
            size,
            message_string: Self::default_message(),
            last_render: Instant::now(),
        }
    }

    pub fn back_to_defaulf(&mut self) {
        self.message_string = Self::default_message();
        self.mark_redraw(true);
    }

    pub fn update_message(&mut self, new_message: &str) {
        self.start_messasge_timer();
        self.message_string = new_message.to_string();
        self.mark_redraw(true);
    }

    // There migh be a weird *BUG* in this function
    pub fn check_message_expired(&mut self, limit: Duration) {
        let now = Instant::now();
        let elapsed = now.duration_since(self.last_render);

        if elapsed >= limit {
            self.message_string = Self::default_message();
            self.mark_redraw(true);
        }
    }

    // ======================================== HELPER METHODS =======================================================

    fn start_messasge_timer(&mut self) {
        self.last_render = Instant::now();
    }

    fn default_message() -> String {
        "HELP: Ctrl-F = find | Ctrl-S = save | Ctrl-Q = quit".to_string()
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
