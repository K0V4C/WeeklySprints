mod buffer;
mod search_info;
mod location;
mod messages;

use std::cmp;

use buffer::Buffer;
use location::Location;
use messages::Message;
use search_info::SearchInfo;

use crate::editor::{
    caret_position::CaretPosition, command::{Edit, Move}, document_status::DocumentStatus, line::Line, terminal::{Terminal, TerminalSize}
};

use super::UiComponent;

const EDITOR_NAME: &str = "HECTO";
const EDITOR_VERSION: &str = env!("CARGO_PKG_VERSION");



pub struct View {
    buffer: Buffer,
    needs_redraw: bool,
    size: TerminalSize,
    text_location: Location,
    scroll_offset: CaretPosition,
    search_info: Option<SearchInfo>,
}

impl View {
    // ======================================= PUBLIC INTERFACE ==================================================

    pub fn new(vertical_margin: usize) -> Self {
        let terminal_size = Terminal::size().unwrap_or_default();

        let margined_size = TerminalSize {
            rows: terminal_size.rows.saturating_sub(vertical_margin),
            columns: terminal_size.columns,
        };

        View {
            buffer: Buffer::default(),
            needs_redraw: true,
            size: margined_size,
            text_location: Location::default(),
            scroll_offset: CaretPosition::default(),
            search_info: Some(SearchInfo::default()),
        }
    }

    pub fn handle_save(&mut self) -> Result<(), std::io::Error> {
        self.save()?;
        Ok(())
    }

    pub fn is_file_given(&self) -> bool {
        self.buffer.is_file_given()
    }

    pub fn handle_move_command(&mut self, move_command: Move) {
        self.move_text_location(move_command);
    }

    pub fn handle_edit_command(&mut self, edit_command: Edit) {
        match edit_command {
            Edit::Tab => self.tab(),
            Edit::Enter => self.enter(),
            Edit::Delete => self.delete_grapheme(),
            Edit::Backspace => self.backspace(),
            Edit::Input(x) => self.add_to_buffer(x),
        }
    }

    pub fn load(&mut self, file_name: &str) {
        self.buffer.load(file_name);
    }

    pub fn set_buffer_file(&mut self, file_name: &str) {
        self.buffer.set_file(file_name);
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
        if self.is_file_given() {
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

    fn move_text_location(&mut self, direction: Move) {
        let TerminalSize { rows, .. } = self.size;

        match direction {
            Move::Up => self.move_up(1),
            Move::Down => self.move_down(1),
            Move::Left => self.move_left(),
            Move::Right => self.move_right(),
            Move::PageUp => self.move_up(rows.saturating_sub(1)),
            Move::PageDown => self.move_down(rows.saturating_sub(1)),
            Move::Home => self.move_to_start_line(),
            Move::End => self.move_to_end_line(),
        }

        self.scroll_text_location_into_view();
    }

    fn add_to_buffer(&mut self, chr: char) {
        let old_len = self
            .buffer
            .data
            .get(self.text_location.line_idx)
            .map_or(0, Line::grapheme_count);

        self.buffer.add_character_at(chr, self.text_location);

        let new_len = self
            .buffer
            .data
            .get(self.text_location.line_idx)
            .map_or(0, Line::grapheme_count);

        let delta = new_len.saturating_sub(old_len);

        if delta > 0 {
            self.move_text_location(Move::Right);
        }
        self.mark_redraw(true);
    }

    fn backspace(&mut self) {
        // Top left does nothing
        if self.text_location.line_idx == 0 && self.text_location.grapheme_idx == 0 {
            return;
        }
        self.move_text_location(Move::Left);
        self.delete_grapheme();
    }

    fn delete_grapheme(&mut self) {
        self.buffer.delete_character_at(self.text_location);
        self.mark_redraw(true);
    }

    fn enter(&mut self) {
        self.buffer.insert_newline(self.text_location);
        self.move_text_location(Move::Down);
        self.move_to_start_line();
        self.mark_redraw(true);
    }

    fn tab(&mut self) {
        self.add_to_buffer('\t');
    }

    fn save(&mut self) -> Result<(), std::io::Error> {
        self.buffer.save()
    }

    // ======================================= SEARCH =================================================================

    pub fn search_next(&mut self, search_string: &str) {
        if let Some(location) = self.next_valid_location() {
            self.query(search_string, location);
        }
    }

    pub fn search(&mut self, search_string: &str) {
        self.query(search_string, self.text_location);
    }

    fn query(&mut self, search_string: &str, start_location: Location) {
        if search_string.is_empty() {
            return;
        }
        if let Some(found) = self.buffer.forward_find(search_string, start_location) {
            self.text_location = found;
            self.scroll_text_location_into_view();
            self.center_text_location();
        }
    }

    pub fn enter_search(&mut self) {
        self.search_info = Some(SearchInfo {
            prev_location: self.text_location,
        });
    }
    
    fn center_text_location(&mut self) {
        let TerminalSize { columns, rows } = self.size;
        let CaretPosition { column, row } = self.text_location_to_position();
        
        let vertical_middle = rows.div_ceil(2);
        let horizontal_middle = columns.div_ceil(2);
        
        self.scroll_offset.row  = row.saturating_sub(vertical_middle);
        self.scroll_offset.column = column.saturating_sub(horizontal_middle);
        
        self.needs_redraw = true;
    }

    pub fn dissmiss_search(&mut self) {
        if let Some(loc) = self.search_info {
            self.text_location = loc.prev_location;
        }

        self.search_info = None;
        self.scroll_text_location_into_view();
    }

    pub fn exit_search(&mut self) {
        self.search_info = None;
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

    fn next_valid_location(&self) -> Option<Location> {
        let line_len = self.buffer.get_line_length(self.text_location.line_idx)?;
        
        let Location {
            grapheme_idx,
            line_idx,
        } = self.text_location;

        let res = if grapheme_idx.saturating_add(1) > line_len
            && line_idx >= self.buffer.get_number_of_lines()
        {
            Location {
                line_idx: 0,
                grapheme_idx: 0,
            }
        } else if grapheme_idx.saturating_add(1) > line_len {
            Location {
                line_idx: line_idx.saturating_add(1),
                grapheme_idx: 0,
            }
        } else {
            Location {
                line_idx,
                grapheme_idx: grapheme_idx.saturating_add(1),
            }
        };
        Some(res)
    }

    // =========================================== SNAPING =====================================================

    fn snap_to_valid_grapheme(&mut self) {
        self.text_location.grapheme_idx = self
            .buffer
            .data
            .get(self.text_location.line_idx)
            .map_or(0, |line| {
                cmp::min(line.grapheme_count(), self.text_location.grapheme_idx)
            });
    }

    fn snap_to_valid_line(&mut self) {
        self.text_location.line_idx = cmp::min(
            self.text_location.line_idx,
            self.buffer.get_number_of_lines(),
        );
    }

    // ======================================== CARET MOVEMENT ===================================================

    fn move_up(&mut self, step: usize) {
        self.text_location.line_idx = self.text_location.line_idx.saturating_sub(step);
        self.snap_to_valid_grapheme();
    }

    fn move_down(&mut self, step: usize) {
        self.text_location.line_idx = self.text_location.line_idx.saturating_add(step);
        self.snap_to_valid_grapheme();
        self.snap_to_valid_line();
    }

    fn move_left(&mut self) {
        if self.text_location.grapheme_idx > 0 {
            self.text_location.grapheme_idx -= 1;
        } else if self.text_location.line_idx > 0 {
            self.move_up(1);
            self.move_to_end_line();
        }
    }

    fn move_right(&mut self) {
        let line_width = self
            .buffer
            .data
            .get(self.text_location.line_idx)
            .map_or(0, Line::grapheme_count);

        if self.text_location.grapheme_idx < line_width {
            self.text_location.grapheme_idx += 1;
        } else {
            self.move_to_start_line();
            self.move_down(1);
        }
    }

    fn move_to_start_line(&mut self) {
        self.text_location.grapheme_idx = 0;
    }

    fn move_to_end_line(&mut self) {
        self.text_location.grapheme_idx = self
            .buffer
            .data
            .get(self.text_location.line_idx)
            .map_or(0, Line::grapheme_count);
    }

    // ===================================== Additional Helpers ===================================================

    fn text_location_to_position(&self) -> CaretPosition {
        let row = self.text_location.line_idx;
        let col = self.buffer.data.get(row).map_or(0, |line| {
            line.width_until(self.text_location.grapheme_idx)
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
