mod buffer;
mod line;
mod messages;

use std::cmp;

use buffer::Buffer;
use line::Line;
use messages::Message;

use crate::editor::{
    document_status::DocumentStatus,
    editor_command::{Direction, EditorCommand},
    terminal::{CaretPosition, Terminal, TerminalSize},
};

use super::UiComponent;

const EDITOR_NAME: &str = "HECTO";
const EDITOR_VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Clone, Copy, Default, Debug)]
pub struct Location {
    pub grapheme_index: usize,
    pub line_index: usize,
}

pub struct View {
    buffer: Buffer,
    needs_redraw: bool,
    size: TerminalSize,
    text_location: Location,
    scroll_offset: CaretPosition,
    file_given: bool,
}

impl View {
    // ======================================= PUBLIC INTERFACE ==================================================

    pub fn new(vertical_margin: usize) -> Self {
        let terminal_size = Terminal::size().unwrap_or_default();

        let margined_size = TerminalSize {
            rows: terminal_size.rows,
            columns: terminal_size.columns.saturating_sub(vertical_margin),
        };

        View {
            buffer: Buffer::default(),
            needs_redraw: true,
            size: margined_size,
            text_location: Location::default(),
            scroll_offset: CaretPosition::default(),
            file_given: false,
        }
    }

    pub fn handle_command(&mut self, command: EditorCommand) {
        match command {
            EditorCommand::Move(direction) => self.move_text_location(&direction),
            EditorCommand::Resize(resize) => self.resize(resize),
            EditorCommand::Input(charater) => self.add_to_buffer(charater),
            EditorCommand::Backspace => self.backspace(),
            EditorCommand::Delete => self.delete_grapheme(),
            EditorCommand::Enter => self.enter(),
            EditorCommand::Tab => self.tab(),
            EditorCommand::Save => self.save(),
            EditorCommand::Quit => {}
        }
    }

    pub fn load(&mut self, file_name: &str) {
        self.buffer.load(file_name);
        self.file_given = true;
    }

    pub fn caret_position(&self) -> CaretPosition {
        self.text_location_to_position()
            .saturating_sub(self.scroll_offset)
    }

    pub fn resize(&mut self, new_size: TerminalSize) {
        self.size = new_size;
        self.scroll_text_location_into_view();
        self.mark_redraw(true);
    }

    pub fn get_status(&self) -> DocumentStatus {
        DocumentStatus {
            caret_position: self.caret_position(),
            file_name: self.buffer.get_file_name(),
            number_of_lines: self.buffer.get_number_of_lines(),
            is_modified: self.buffer.is_modified(),
        }
    }

    // ============================================ RENDERING =====================================================

    fn draw_rows(&mut self) -> Result<(), std::io::Error> {
        let _ = Terminal::move_caret_to(CaretPosition { column: 0, row: 0 });
        for row in 0..self.size.rows {
            Terminal::print_row(row, "~")?;
        }
        let _ = Terminal::move_caret_to(CaretPosition { column: 0, row: 0 });
        Ok(())
    }

    fn draw_buffer(&self) -> Result<(), std::io::Error> {
        let (width, height) = (self.size.columns, self.size.rows);

        if width == 0 || height == 0 {
            return Ok(());
        }

        let top = self.scroll_offset.row;

        for current_row in 0..height {
            if let Some(line) = self.buffer.data.get(current_row.saturating_add(top)) {
                let left = self.scroll_offset.column;
                let right = self.scroll_offset.column.saturating_add(width);
                Terminal::print_row(current_row, &line.get_visable_graphemes(left..right))?;
            }
        }
        Ok(())
    }

    fn draw_welcome_message(&self) -> Result<(), std::io::Error> {
        // File and no welcome
        if self.file_given {
            return Ok(());
        }

        let welcome_message_buffer =
            Message::build_welcome_message(self.size, EDITOR_NAME, EDITOR_VERSION);

        let mut start_render_line = self.size.rows / 3;

        for line in welcome_message_buffer.data {
            // Cut off is here if someone wants to build different welcome message they dont have to worry about it fitting perfectly
            Terminal::print_row(
                start_render_line,
                &line.get_visable_graphemes(0..self.size.columns),
            )?;
            start_render_line += 1;
        }

        Ok(())
    }

    // ========================================= COMMAND HANDLING ==============================================

    fn move_text_location(&mut self, direction: &Direction) {
        let TerminalSize { rows, .. } = self.size;

        match direction {
            Direction::Up => self.move_up(1),
            Direction::Down => self.move_down(1),
            Direction::Left => self.move_left(),
            Direction::Right => self.move_right(),
            Direction::PageUp => self.move_up(rows.saturating_sub(1)),
            Direction::PageDown => self.move_down(rows.saturating_sub(1)),
            Direction::Home => self.move_to_start_line(),
            Direction::End => self.move_to_end_line(),
        }

        self.scroll_text_location_into_view();
    }

    fn add_to_buffer(&mut self, chr: char) {
        let old_len = self
            .buffer
            .data
            .get(self.text_location.line_index)
            .map_or(0, Line::grapheme_count);

        self.buffer.add_character_at(chr, self.text_location);

        let new_len = self
            .buffer
            .data
            .get(self.text_location.line_index)
            .map_or(0, Line::grapheme_count);

        let delta = new_len.saturating_sub(old_len);

        if delta > 0 {
            self.move_text_location(&Direction::Right);
        }
        self.mark_redraw(true);
    }

    fn backspace(&mut self) {
        // Top left does nothing
        if self.text_location.line_index == 0 && self.text_location.grapheme_index == 0 {
            return;
        }
        self.move_text_location(&Direction::Left);
        self.delete_grapheme();
    }

    fn delete_grapheme(&mut self) {
        self.buffer.delete_character_at(self.text_location);
        self.mark_redraw(true);
    }

    fn enter(&mut self) {
        self.buffer.insert_newline(self.text_location);
        self.move_text_location(&Direction::Down);
        self.mark_redraw(true);
    }

    fn tab(&mut self) {
        self.add_to_buffer('\t');
    }

    fn save(&mut self) {
        let _ = self.buffer.save();
    }

    // =========================================== SCROLLING ===================================================
    fn scroll_text_location_into_view(&mut self) {
        let CaretPosition { column, row } = self.text_location_to_position();

        self.scroll_vertical(row);
        self.scroll_horizontal(column);
    }

    fn scroll_horizontal(&mut self, to: usize) {
        let TerminalSize { columns, .. } = self.size;

        let offset_changed = if to < self.scroll_offset.column {
            self.scroll_offset.column = to;
            true
        } else if to >= self.scroll_offset.column.saturating_add(columns) {
            self.scroll_offset.column = to.saturating_sub(columns).saturating_add(1);
            true
        } else {
            false
        };

        if offset_changed {
            self.mark_redraw(offset_changed);
        };
    }

    fn scroll_vertical(&mut self, to: usize) {
        let TerminalSize { rows, .. } = self.size;

        let offset_changed = if to < self.scroll_offset.row {
            self.scroll_offset.row = to;
            true
        } else if to >= self.scroll_offset.row.saturating_add(rows) {
            self.scroll_offset.row = to.saturating_sub(rows).saturating_add(1);
            true
        } else {
            false
        };

        if offset_changed {
            self.mark_redraw(offset_changed);
        };
    }

    // =========================================== HELPERS =====================================================

    // =========================================== SNAPING =====================================================

    fn snap_to_valid_grapheme(&mut self) {
        self.text_location.grapheme_index = self
            .buffer
            .data
            .get(self.text_location.line_index)
            .map_or(0, |line| {
                cmp::min(line.grapheme_count(), self.text_location.grapheme_index)
            });
    }

    fn snap_to_valid_line(&mut self) {
        self.text_location.line_index = cmp::min(
            self.text_location.line_index,
            self.buffer.get_number_of_lines(),
        );
    }

    // ======================================== CARET MOVEMENT ===================================================

    fn move_up(&mut self, step: usize) {
        self.text_location.line_index = self.text_location.line_index.saturating_sub(step);
        self.snap_to_valid_grapheme();
    }

    fn move_down(&mut self, step: usize) {
        self.text_location.line_index = self.text_location.line_index.saturating_add(step);
        self.snap_to_valid_grapheme();
        self.snap_to_valid_line();
    }

    fn move_left(&mut self) {
        if self.text_location.grapheme_index > 0 {
            self.text_location.grapheme_index -= 1;
        } else if self.text_location.line_index > 0 {
            self.move_up(1);
            self.move_to_end_line();
        }
    }

    fn move_right(&mut self) {
        let line_width = self
            .buffer
            .data
            .get(self.text_location.line_index)
            .map_or(0, Line::grapheme_count);

        if self.text_location.grapheme_index < line_width {
            self.text_location.grapheme_index += 1;
        } else {
            self.move_to_start_line();
            self.move_down(1);
        }
    }

    fn move_to_start_line(&mut self) {
        self.text_location.grapheme_index = 0;
    }

    fn move_to_end_line(&mut self) {
        self.text_location.grapheme_index = self
            .buffer
            .data
            .get(self.text_location.line_index)
            .map_or(0, Line::grapheme_count);
    }

    // ===================================== Additional Helpers ===================================================

    fn text_location_to_position(&self) -> CaretPosition {
        let row = self.text_location.line_index;
        let col = self.buffer.data.get(row).map_or(0, |line| {
            line.width_until(self.text_location.grapheme_index)
        });
        CaretPosition { column: col, row }
    }
}

impl UiComponent for View {
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
    fn draw(&mut self, _: usize) -> Result<(), std::io::Error> {
        self.draw_rows()?;
        self.draw_buffer()?;
        self.draw_welcome_message()?;
        Ok(())
    }
}
