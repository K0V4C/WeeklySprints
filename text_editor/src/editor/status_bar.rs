use super::{
    document_status::DocumentStatus, terminal::{Terminal, TerminalSize}
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

        let number_of_lines = self.status.line_count_to_string();
        let position = self.status.position_indicator_to_string();
        let modification = self.status.modified_indicator_to_string();

        let line = format!("{:<.50} - {} lines {}", file_name, number_of_lines, modification);

        let padding_left = self.width.saturating_sub(line.len()).saturating_sub(position.len()).saturating_sub(3);
        let padding_right = 3;
        let line = format!("{line}{}{position}{}", " ".repeat(padding_left), " ".repeat(padding_right));

        Self::render_line(self.get_rendering_row(), &line);
    }

    // ================================================= RENDERING ==============================================

    fn get_rendering_row(&self) -> usize {
        Terminal::size()
            .unwrap_or_default()
            .rows
            .saturating_sub(self.height() + 1)
    }

    fn render_line(row: usize, string_to_render: &str) {
        let result = Terminal::print_row_with_attribute(row, crossterm::style::Attribute::Reverse, string_to_render);
        debug_assert!(result.is_ok(), "Failed to render the line!");
    }
}
