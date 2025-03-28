mod buffer;
mod line;
mod messages;

use std::cmp;

use buffer::Buffer;
use line::Line;
use messages::Message;

use super::{
    editor_command::{Direction, EditorCommand},
    terminal::{CaretPosition, Terminal, TerminalSize},
};

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
    file_given: bool
}

impl Default for View {
    fn default() -> Self {
        View {
            buffer: Buffer::default(),
            needs_redraw: true,
            size: Terminal::size().unwrap_or_default(),
            text_location: Location::default(),
            scroll_offset: CaretPosition::default(),
            file_given: false
        }
    }
}

impl View {
    // ======================================= PUBLIC INTERFACE ==================================================

    pub fn render(&mut self) {
        if !self.needs_redraw {
            return;
        }

        self.draw_rows();
        self.draw_buffer();
        self.draw_welcome_message();

        self.needs_redraw = false;
    }

    pub fn handle_command(&mut self, command: EditorCommand) {
        match command {
            EditorCommand::Move(direction) => self.move_text_location(&direction),
            EditorCommand::Resize(size) => self.resize(size),
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
        if let Ok(context) = std::fs::read_to_string(file_name) {
            self.buffer.clear();
            self.buffer.file_name = file_name.to_string();

            for line in context.lines() {
                self.buffer.push(line);
            }
        }
        self.file_given = true;
    }

    pub fn caret_position(&self) -> CaretPosition {
        self.text_location_to_position()
            .saturating_sub(self.scroll_offset)
    }

    pub fn resize(&mut self, new_size: TerminalSize) {
        self.size = new_size;
        self.scroll_text_location_into_view();
        self.needs_redraw = true;
    }

    // ============================================ RENDERING =====================================================

    fn draw_rows(&mut self) {
        let _ = Terminal::move_caret_to(CaretPosition { column: 0, row: 0 });
        for row in 0..self.size.rows {
            Self::render_line(row, "~");
        }
        let _ = Terminal::move_caret_to(CaretPosition { column: 0, row: 0 });
    }

    fn draw_buffer(&self) {
        let (width, height) = (self.size.columns, self.size.rows);

        if width == 0 || height == 0 {
            return;
        }

        let top = self.scroll_offset.row;

        for current_row in 0..height {
            if let Some(line) = self.buffer.data.get(current_row.saturating_add(top)) {
                let left = self.scroll_offset.column;
                let right = self.scroll_offset.column.saturating_add(width);

                Self::render_line(current_row, &line.get_visable_graphemes(left..right));
            }
        }
    }

    fn draw_welcome_message(&self) {
        // File and no welcome
        if self.file_given {
            return;
        }

        let welcome_message_buffer =
            Message::build_welcome_message(self.size, EDITOR_NAME, EDITOR_VERSION);

        let mut start_render_line = self.size.rows / 3;

        for line in welcome_message_buffer.data {
            // Cut off is here if someone wants to build different welcome message they dont have to worry about it fitting perfectly
            Self::render_line(
                start_render_line,
                &line.get_visable_graphemes(0..self.size.columns),
            );
            start_render_line += 1;
        }
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
        self.needs_redraw = true;
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
        self.needs_redraw = true;
    }

    fn enter(&mut self) {
        self.buffer.insert_newline(self.text_location);
        self.move_text_location(&Direction::Down);
        self.needs_redraw = true;
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
            self.needs_redraw = offset_changed;
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
            self.needs_redraw = offset_changed;
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

    fn render_line(row: usize, string_to_render: &str) {
        let result = Terminal::print_row(row, string_to_render);
        debug_assert!(result.is_ok(), "Failed to render the line!");
    }
}
