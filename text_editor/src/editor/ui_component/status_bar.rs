use crate::editor::{document_status::DocumentStatus, size::Size, terminal::Terminal};

use super::UiComponent;

#[derive(Default)]
pub struct StatusBar {
    status: DocumentStatus,
    needs_redraw: bool,
    size: Size,
}

/// It is assumed for this component to be of size 1 vertically
impl StatusBar {
    // ======================================== PUBLIC INTERFACE ==================================================
    pub fn new() -> Self {
        let size = Terminal::size().unwrap_or_default();

        StatusBar {
            status: DocumentStatus::default(),
            needs_redraw: true,
            size,
        }
    }

    pub fn update_status(&mut self, status: DocumentStatus) {
        self.status = status;
        self.mark_redraw(true);
    }

    // =========================================      HELPER     ==================================================

    fn build_document_status_string(&self) -> String {
        let file_name = self
            .status
            .file_name
            .clone()
            .unwrap_or("[None]".to_string());

        let number_of_lines = self.status.line_count_to_string();
        let position = self.status.position_indicator_to_string();
        let modification = self.status.modified_indicator_to_string();

        let line = format!("{file_name:<.50} - {number_of_lines} lines {modification}",);

        let padding_left = self
            .size
            .columns
            .saturating_sub(line.len())
            .saturating_sub(position.len())
            .saturating_sub(3);
        let padding_right = 3;

        format!(
            "{line}{}{position}{}",
            " ".repeat(padding_left),
            " ".repeat(padding_right)
        )
    }
}

impl UiComponent for StatusBar {
    /// Marks if ui component need to be redrawn
    fn mark_redraw(&mut self, needs_redraw: bool) {
        self.needs_redraw = needs_redraw;
    }

    /// Get status of redraw
    fn needs_redraw(&self) -> bool {
        self.needs_redraw
    }

    /// Set the size of the component
    fn set_size(&mut self, new_size: Size) {
        self.size = new_size;
    }

    /// Method to actually draw the component, must be implemented by each component
    fn draw(&mut self, origin_y: usize) -> Result<(), std::io::Error> {
        let msg = self.build_document_status_string();
        Terminal::print_row_with_attribute(origin_y, crossterm::style::Attribute::Reverse, &msg)
    }
}
