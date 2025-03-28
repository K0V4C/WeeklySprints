use super::{
    DocumentStatus,
    terminal::{Terminal, TerminalSize},
};

#[derive(Default)]
pub struct StatusBar {
    status: DocumentStatus,
    needs_redraw: bool,
    width: usize,
    height: usize,
}

impl StatusBar {
    // ======================================== PUBLIC INTERFACE ==================================================
    pub fn new(vertical_size: usize) -> Self {
        let size = Terminal::size().unwrap_or_default();

        StatusBar {
            status: DocumentStatus::default(),
            needs_redraw: true,
            width: size.columns,
            height: vertical_size,
        }
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn update_status(&mut self, status: DocumentStatus) {
        self.status = status;
        self.needs_redraw = true;
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

        self.render_document_status();

        self.needs_redraw = false;
    }

    // =========================================      HELPER     ==================================================

    fn render_document_status(&self) {
        let file_name = self
            .status
            .file_name
            .clone()
            .unwrap_or("[None]".to_string());

        let number_of_lines = self.status.number_of_lines;

        let x = self.status.caret_position.row;
        let y = self.status.caret_position.column;
        let position = format!("{x}/{y}");

        let modification = if self.status.is_modified { "M" } else { "N" };

        let line = format!("{file_name} {number_of_lines}  {position} {modification} ");

        Self::render_line(self.get_rendering_row(), &line);
    }

    fn get_rendering_row(&self) -> usize {
        Terminal::size()
            .unwrap_or_default()
            .rows
            .saturating_sub(self.height() + 1)
    }

    fn render_line(row: usize, string_to_render: &str) {
        let result = Terminal::print_row(row, string_to_render);
        debug_assert!(result.is_ok(), "Failed to render the line!");
    }
}
