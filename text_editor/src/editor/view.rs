mod buffer;
mod line;
mod location;
mod messages;

use buffer::Buffer;
use location::Location;
use messages::Message;

use super::{editor_command::{Direction, EditorCommand}, terminal::{CaretPosition, Terminal, TerminalSize}};

const EDITOR_NAME: &str = "HECTO";
const EDITOR_VERSION: &str = env!("CARGO_PKG_VERSION");

pub struct View {
    buffer: Buffer,
    needs_redraw: bool,
    size: TerminalSize,
    location: Location,
    scroll_offset: Location
}

impl Default for View {
    fn default() -> Self {
        View {
            buffer: Buffer::default(),
            needs_redraw: true,
            size: Terminal::size().unwrap_or_default(),
            location: Location::default(),
            scroll_offset: Location::default()
        }
    }
}

impl View {
    pub fn render(&mut self) {
        if !self.needs_redraw {
            return;
        }

        self.draw_rows();
        self.dump_buffer();
        self.draw_welcome_message();

        self.needs_redraw = false;
    }

    pub fn handle_command(&mut self, command: EditorCommand) {
        match command {
            EditorCommand::Move(direction) => self.move_text_location(&direction),
            EditorCommand::Resize(size) => self.resize(size),
            EditorCommand::Quit => {}
        }
    }

    pub fn resize(&mut self, new_size: TerminalSize) {
        self.size = new_size;
        self.scroll_location_into_view();
        self.needs_redraw = true;
    }

    pub fn get_position(&self) -> CaretPosition {
        self.location.substract(&self.scroll_offset).into()
    }

    fn move_text_location(&mut self, direction: &Direction) {
        let TerminalSize { columns, rows } = Terminal::size().unwrap_or_default();
        let Location { mut x, mut y } = self.location;

        match direction {
            Direction::Left => {
                x = x.saturating_sub(1);
            }

            Direction::Right => {
                x = x.saturating_add(1);
            }

            Direction::Up => {
                y = y.saturating_sub(1);
            }

            Direction::Down => {
                y = y.saturating_add(1);
            }

            Direction::Home => {
                x = 0;
            }

            Direction::End => {
                x = columns.saturating_sub(1);
            }

            Direction::PageUp => {
                y = 0;
            }

            Direction::PageDown => {
                y = rows.saturating_sub(1);
            }
        }

        self.location = Location { x, y };
        self.scroll_location_into_view();
    }

    fn scroll_location_into_view(&mut self) {

        let Location { x , y } = self.location;
        let TerminalSize { columns, rows } = self.size;

        let mut offset_changed = false;

        // This logic took way too long to figure out

        // Scroll vertically
        if y < self.scroll_offset.y {
            self.scroll_offset.y = y;
            offset_changed = true;
        } else if y >= self.scroll_offset.y.saturating_add(rows) {
            self.scroll_offset.y = y.saturating_sub(rows).saturating_add(1);
            offset_changed = true;
        }

        // Scroll horizontaly
        if x < self.scroll_offset.x {
            self.scroll_offset.x = x;
            offset_changed = true;
        } else if x >= self.scroll_offset.x.saturating_add(columns) {
            self.scroll_offset.x = x.saturating_sub(columns).saturating_add(1);
            offset_changed = true;
        }

        self.needs_redraw = offset_changed;
    }

    fn draw_welcome_message(&self) {

        // If buffer is empty we just draw welcome message
        if !self.buffer.is_empty() {
            return;
        }

        let welcome_message_buffer = Message::build_welcome_message(self.size, EDITOR_NAME, EDITOR_VERSION);

        let mut start_render_line = self.size.rows / 3;

        for line in welcome_message_buffer.data {
            // Cut off is here if someone wants to build different welcome message they dont have to worry about it fitting perfectly
            Self::render_line(start_render_line, &line.get(0..self.size.columns));
            start_render_line += 1;
        }
    }

    fn draw_rows(&self) {
        Self::caret_to_start();
        for row in 0..self.size.rows {
            Self::render_line(row, "~");
        }
        Self::caret_to_start();
    }

    pub fn load(&mut self, file_name: &str) {
        if let Ok(context) = std::fs::read_to_string(file_name) {
            self.buffer.clear();

            for line in context.lines() {
                self.buffer.push(line);
            }
        }
    }

    fn dump_buffer(&self) {

        let (width, height) = (self.size.columns, self.size.rows);

        let top = self.scroll_offset.y;

        for current_row in 0..height {
            if let Some(line) = self.buffer.data.get(current_row.saturating_add(top)) {
                let left = self.scroll_offset.x;
                let right = self.scroll_offset.x.saturating_add(width);
                Self::render_line(current_row, &line.get(left..right));
            }
        }
    }


    fn render_line(row: usize, string_to_render: &str) {
        let result = Terminal::print_row(row, string_to_render);
        debug_assert!(result.is_ok(), "Failed to render the line!");
    }

    fn caret_to_start() {
        let result = Terminal::move_caret_to(CaretPosition { column: 0, row: 0 });
        debug_assert!(result.is_ok(), "Failed to move caret to start position");
    }
}
