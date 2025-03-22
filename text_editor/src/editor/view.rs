mod buffer;

use buffer::Buffer;

use super::terminal::{CaretPosition, Terminal, TerminalSize};

const EDITOR_NAME: &str = "HECTO";
const EDITOR_VERSION: &str = env!("CARGO_PKG_VERSION");

pub struct View {
    buffer: Buffer,
    needs_redraw: bool,
    size: TerminalSize,
}

impl Default for View {
    fn default() -> Self {
        View {
            buffer: Buffer::default(),
            needs_redraw: true,
            size: Terminal::size().unwrap_or_default(),
        }
    }
}

impl View {
    pub fn render(&mut self) -> Result<(), std::io::Error> {
        if !self.needs_redraw {
            return Ok(());
        }

        self.draw_rows()?;
        self.dump_buffer()?;
        self.draw_welcome_message()?;

        self.needs_redraw = false;

        Ok(())
    }

    pub fn resize(&mut self, new_size: TerminalSize) {
        self.needs_redraw = true;
        self.size = new_size;
    }

    fn draw_rows(&self) -> Result<(), std::io::Error> {
        Self::caret_to_start()?;
        for row in 0..self.size.rows {
            Self::render_line(row, "~")?;
        }
        Self::caret_to_start()?;

        Ok(())
    }

    pub fn load(&mut self, file_name: &str) -> Result<(), std::io::Error> {
        let context = std::fs::read_to_string(file_name)?;

        self.buffer.clear();

        for line in context.lines() {
            self.buffer.push(line.to_string());
        }

        Ok(())
    }

    fn dump_buffer(&self) -> Result<(), std::io::Error> {
        for (row, line_data) in self.buffer.data.iter().enumerate() {
            let width = self.size.columns;
            let truncated_line = if line_data.len() < width {
                line_data
            } else {
                &line_data[0..width]
            };

            Self::render_line(row, truncated_line)?;
        }

        Ok(())
    }

    /// Draws welcome message only if nothing else was passed to the buffer
    fn draw_welcome_message(&mut self) -> Result<(), std::io::Error> {
        // If buffer is empty we just draw welcome message
        if !self.buffer.is_empty() {
            return Ok(());
        }

        let mut render_row = self.size.rows / 3;
        let length_of_terminal = self.size.columns;

        let blank_line =
            String::from("|") + &" ".repeat(length_of_terminal.saturating_sub(2)) + "|";

        // Draw top bar
        Self::render_line(render_row, &"=".repeat(length_of_terminal))?;
        render_row += 1;

        // Draw sides
        for _ in 0..5 {
            Self::render_line(render_row, &blank_line)?;
            render_row += 1;
        }

        // Draw Name and version number
        let blank_space =
            (length_of_terminal - EDITOR_NAME.len() - EDITOR_VERSION.len()).saturating_sub(3);

        let (padding_left, padding_right) = if blank_space % 2 == 0 {
            (&" ".repeat(blank_space / 2), &" ".repeat(blank_space / 2))
        } else {
            (
                &" ".repeat(blank_space / 2 + 1),
                &" ".repeat(blank_space / 2),
            )
        };

        let center_line = format!("|{padding_left}{EDITOR_NAME} {EDITOR_VERSION}{padding_right}|");

        if center_line.len() >= self.size.columns {
            Self::render_line(render_row, &center_line)?;
        } else {
            Self::render_line(render_row, &blank_line)?;
        }

        render_row += 1;
        // Draw sides
        for _ in 0..5 {
            Self::render_line(render_row, &blank_line)?;
            render_row += 1;
        }

        // Draw top bar
        Self::render_line(render_row, &"=".repeat(length_of_terminal))?;

        Ok(())
    }

    fn render_line(row: usize, string_to_render: &str) -> Result<(), std::io::Error> {
        Terminal::move_caret_to(CaretPosition { column: 0, row })?;
        Terminal::clear_line()?;
        Terminal::print(string_to_render)?;
        Ok(())
    }

    fn caret_to_start() -> Result<(), std::io::Error> {
        Terminal::move_caret_to(CaretPosition { column: 0, row: 0 })?;
        Ok(())
    }
}
