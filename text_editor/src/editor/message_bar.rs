use super::terminal::{Terminal, TerminalSize};


#[derive(Default)]
pub struct MessageBar {
    needs_redraw: bool,
    width: usize,
    height: usize,
}

impl MessageBar {
    // ======================================== PUBLIC INTERFACE ==================================================
    pub fn new(vertical_size: usize) -> Self {
        let size = Terminal::size().unwrap_or_default();

        MessageBar {
            needs_redraw: true,
            width: size.columns,
            height: vertical_size,
        }
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn resize(&mut self, new_size: TerminalSize) {
        self.width = new_size.columns;
        self.height = new_size.rows;
        self.needs_redraw = true;
    }

    pub fn render(&mut self) {
        if !self.needs_redraw {
            return;
        }


        self.needs_redraw = false;
    }

    // ======================================== HELPER METHODS =======================================================
    
    
    
    // ================================================= RENDERING ==============================================
    
    /// This wants to get rendered at the bottom of terminal
    fn get_rendering_row(&self) -> usize {
        Terminal::size()
            .unwrap_or_default()
            .rows
            .saturating_sub(self.height())
    }

    fn render_line(row: usize, string_to_render: &str) {
        let result = Terminal::print_row(row, string_to_render);
        debug_assert!(result.is_ok(), "Failed to render the line!");
    }
}
