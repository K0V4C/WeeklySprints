mod buffer;
mod highlighter;
pub mod location;
mod messages;
pub mod search_info;

use std::cmp;

use buffer::Buffer;
use highlighter::Highlighter;
use location::Location;
use messages::Message;
use search_info::SearchInfo;

use crate::editor::{
    caret_position::CaretPosition,
    command::{edit::Edit, movement::Move},
    document_status::DocumentStatus,
    line::Line,
    size::Size,
    terminal::Terminal,
};

use super::UiComponent;

const EDITOR_NAME: &str = "HECTO";
const EDITOR_VERSION: &str = env!("CARGO_PKG_VERSION");

pub struct View {
    buffer: Buffer,
    needs_redraw: bool,
    size: Size,
    text_location: Location,
    scroll_offset: CaretPosition,
    search_info: Option<SearchInfo>,
}

impl View {
    // ======================================= PUBLIC INTERFACE ==================================================

    pub fn new(vertical_margin: usize) -> Self {
        let terminal_size = Terminal::size().unwrap_or_default();

        let margined_size = Size {
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

    pub fn resize(&mut self, new_size: Size) {
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

    fn draw_buffer(&self, origin_y: usize) -> Result<(), std::io::Error> {
        let (width, height) = (self.size.columns, self.size.rows);

        if width == 0 || height == 0 {
            return Ok(());
        }
        
        let end_y = origin_y.saturating_add(height);
        let top = self.scroll_offset.row;

        let selected_match = self.search_info.is_some().then_some(self.text_location);
        let query = self
            .search_info
            .as_ref()
            .map(|x| x.search_query.to_string());

        let mut highlighter = Highlighter::new(query, selected_match);

        // It has to be 0 here because of comment blocks
        for current_row in 0..self.buffer.get_number_of_lines() {
            if let Some(line) = self.buffer.get_line(current_row) {
                highlighter.highlight(current_row, line);
            }
        }
        
        
        // This here can start from origin no problems
        for current_row in origin_y..end_y {
            
            
            let line_idx = current_row.saturating_add(top).saturating_sub(origin_y);
            let left = self.scroll_offset.column;
            let right = self.scroll_offset.column.saturating_add(width);
            
            if let Some(annotated_string) =
                self.buffer
                    .get_highlighted_line(line_idx, left..right, &highlighter)
            {
                Terminal::print_annoted_line(current_row, annotated_string)?;
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
        let Size { rows, .. } = self.size;

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

    pub fn search_previous(&mut self, search_string: &str) {
        let before_location = Location {
            grapheme_idx: self.text_location.grapheme_idx.saturating_sub(1),
            line_idx: self.text_location.line_idx,
        };

        self.reverse_query(search_string, before_location);
    }

    pub fn search_next(&mut self, search_string: &str) {
        self.query(
            search_string,
            self.buffer
                .next_valid_search_location(self.text_location, search_string),
        );
    }

    pub fn search(&mut self, search_string: &str) {
        // This is a place for optimisation in case of Search
        self.search_info
            .as_mut()
            .map(|x| x.search_query = Line::from(search_string));

        self.query(search_string, self.text_location);
    }

    fn query(&mut self, search_string: &str, start_location: Location) {
        if search_string.is_empty() {
            return;
        }

        let result = self.buffer.forward_find(search_string, start_location);

        if let Some(found) = result {
            self.text_location = found;
            self.scroll_text_location_into_view();
            self.center_text_location();
        }
    }

    fn reverse_query(&mut self, search_string: &str, start_location: Location) {
        if search_string.is_empty() {
            return;
        }

        let result = self.buffer.backward_find(search_string, start_location);

        if let Some(found) = result {
            self.text_location = found;
            self.scroll_text_location_into_view();
            self.center_text_location();
        }
    }

    pub fn enter_search(&mut self) {
        self.search_info = Some(SearchInfo {
            prev_location: self.text_location,
            search_query: Line::from(""),
        });
    }

    fn center_text_location(&mut self) {
        let Size { columns, rows } = self.size;
        let CaretPosition { column, row } = self.text_location_to_position();

        let vertical_middle = rows.div_ceil(2);
        let horizontal_middle = columns.div_ceil(2);

        self.scroll_offset.row = row.saturating_sub(vertical_middle);
        self.scroll_offset.column = column.saturating_sub(horizontal_middle);

        self.needs_redraw = true;
    }

    pub fn dissmiss_search(&mut self) {
        if let Some(loc) = self.search_info.as_ref() {
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
        let Size { columns, .. } = self.size;

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
        let Size { rows, .. } = self.size;

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
        let col = self
            .buffer
            .data
            .get(row)
            .map_or(0, |line| line.width_until(self.text_location.grapheme_idx));
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
    fn set_size(&mut self, new_size: Size) {
        self.size = new_size;
    }

    /// Method to actually draw the component, must be implemented by each component
    fn draw(&mut self, origin_y: usize) -> Result<(), std::io::Error> {
        self.draw_rows()?;
        self.draw_buffer(origin_y)?;
        self.draw_welcome_message()?;
        Ok(())
    }
}
