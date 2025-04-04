use crate::editor::{
    command::edit::Edit,
    line::Line,
    terminal::Terminal,
    size::Size
};

use super::UiComponent;

pub struct CommandBar {
    needs_redraw: bool,
    size: Size,
    command_line: Line,
    prompt: String,
}

impl CommandBar {
    // ======================================== PUBLIC INTERFACE ==================================================
    pub fn new() -> Self {
        let size = Terminal::size().unwrap_or_default();

        CommandBar {
            needs_redraw: true,
            size,
            command_line: Line::default(),
            prompt: String::new(),
        }
    }

    pub fn handle_edit_command(&mut self, command: Edit) {
        match command {
            Edit::Input(character) => self.add_to_buffer(character),
            Edit::Backspace => self.delete(),
            _ => {}
        }
        self.mark_redraw(true);
    }

    fn add_to_buffer(&mut self, chr: char) {
        self.command_line
            .add_character_to_line(chr, self.command_line.grapheme_count());
    }

    fn delete(&mut self) {
        self.command_line
            .delete_character(self.command_line.grapheme_count().saturating_sub(1));
    }

    pub fn get_command_line(&self) -> String {
        self.command_line.to_string()
    }

    pub fn clear_line(&mut self) {
        self.command_line.clear();
        // self.command_line = Line::default();
        self.mark_redraw(true);
    }

    pub fn set_prompt(&mut self, prompt: String) {
        self.prompt = prompt;
        self.mark_redraw(true);
    }

    pub fn caret_position_column(&self) -> usize {
        let x = self
            .prompt
            .len()
            .saturating_add(self.command_line.grapheme_count());

        std::cmp::min(x, self.size.columns)
    }

    pub fn get_line(&self) -> String {
        self.command_line.to_string()
    }
}

impl UiComponent for CommandBar {
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
        let area_for_value = self.size.columns.saturating_sub(self.prompt.len());
        let value_end = self.command_line.grapheme_count();
        let value_start = value_end.saturating_sub(area_for_value);
        let message = format!(
            "{}{}",
            self.prompt,
            self.command_line
                .get_visable_graphemes(value_start..value_end)
        );
        let to_print = if message.len() <= self.size.columns {
            message
        } else {
            String::new()
        };
        Terminal::print_row(origin_y, &to_print)
    }
}
