use crate::editor::{
    command::{Edit, Move, System},
    terminal::{CaretPosition, Terminal, TerminalSize},
};

use super::{UiComponent, view::line::Line};

const SAVE_AS_STRING: &str = "Save as: ";
const SEARCH_STRING: &str = "Search: ";

enum CommandMode {
    None,
    Saving(String),
    Searching(String),
}

pub struct CommandBar {
    needs_redraw: bool,
    size: TerminalSize,
    caret_position: CaretPosition,
    command_line: Line,
    result: String,
    mode: CommandMode,
}

impl CommandBar {
    // ======================================== PUBLIC INTERFACE ==================================================
    pub fn new() -> Self {
        let size = Terminal::size().unwrap_or_default();

        let caret_position = CaretPosition {
            row: 0,                             // this is relative to only this component so this is true
            column: 0,                          // Never can caret go behind this
        };

        CommandBar {
            needs_redraw: true,
            size,
            caret_position,
            command_line: Line::default(),
            result: String::new(),
            mode: CommandMode::None,
        }
    }

    pub fn get_command_line(&self) -> String {
        self.result.clone()
    }

    pub fn handle_system_command(&mut self, system_command: System) {
        match system_command {
            System::Save => self.mode = CommandMode::Saving(SAVE_AS_STRING.to_string()),
            System::Search => self.mode = CommandMode::Searching(SEARCH_STRING.to_string()),
            System::Abort => self.mode = CommandMode::None,
            _ => (),
        }
        self.command_line.clear();
        self.snap_caret();
        self.mark_redraw(true);
    }

    fn snap_caret(&mut self) {
        let left_limit = self.get_len_left_margin();

        self.caret_position = CaretPosition {
            column: left_limit,
            row: self.caret_position().row
        }
    }

    pub fn handle_edit_command(&mut self, edit_command: Edit) {
        match edit_command {
            Edit::Enter => self.enter(), // This one is handeled by the editor
            Edit::Tab => self.tab(),
            Edit::Delete => self.delete_grapheme(),
            Edit::Backspace => self.backspace(),
            Edit::Input(x) => self.add_to_line(x),
        }
        self.mark_redraw(true);
    }

    fn line_to_string(&self) -> String {
        self.command_line
            .get_visable_graphemes(0..self.command_line.grapheme_count())
    }

    fn enter(&mut self) {
        self.result = self.line_to_string();
    }

    fn tab(&mut self) {
        self.add_to_line('\t');
    }

    fn delete_grapheme(&mut self) {
        let margin = self.get_len_left_margin();
        self.command_line
            .delete_character(self.caret_position().column.saturating_sub(margin));
    }

    fn backspace(&mut self) {
        if self.caret_position().column == self.get_len_left_margin() {
            return;
        }
        self.handle_move_command(Move::Left);
        self.delete_grapheme();
    }

    fn add_to_line(&mut self, x: char) {
        self.command_line
            .add_character_to_line(x, self.caret_position().column);
        self.move_right();
    }

    pub fn handle_move_command(&mut self, direction: Move) {
        match direction {
            Move::Left => self.move_left(),
            Move::Right => self.move_right(),
            Move::Home => self.move_to_start(),
            Move::End => self.move_to_end(),
            _ => (),
        }
    }

    fn move_left(&mut self) {
        let position = self.caret_position;

        let left_limit = self.get_len_left_margin();

        if self.caret_position().column > left_limit {
            self.caret_position = CaretPosition {
                row: position.row,
                column: position.column.saturating_sub(1),
            };
        }
    }

    fn get_len_left_margin(&mut self) -> usize {
        match &self.mode {
            CommandMode::Saving(x) | CommandMode::Searching(x) => x.len(),
            CommandMode::None => 0,
        }
    }

    fn construct_line(&self) -> String {
        match &self.mode {
            CommandMode::Saving(x) | CommandMode::Searching(x) => x.clone() + &self.line_to_string(),
            CommandMode::None => String::new(),
        }
    }

    fn move_right(&mut self) {
        let position = self.caret_position;

        let mut right_limit = if self.command_line.grapheme_count() > self.size.columns {
            self.size.columns
        } else {
            self.command_line.grapheme_count()
        };

        right_limit = right_limit.saturating_add(self.get_len_left_margin());

        if self.caret_position().column < right_limit {
            self.caret_position = CaretPosition {
                row: position.row,
                column: position.column.saturating_add(1),
            };
        }
    }
    fn move_to_start(&mut self) {
        let position = self.caret_position;

        let mut right_limit = if self.command_line.grapheme_count() > self.size.columns {
            self.size.columns
        } else {
            self.command_line.grapheme_count()
        };

        right_limit = right_limit.saturating_add(self.get_len_left_margin());

        self.caret_position = CaretPosition {
            row: position.row,
            column: right_limit,
        };
    }
    fn move_to_end(&mut self) {
        let position = self.caret_position;

        self.caret_position = CaretPosition {
            row: position.row,
            column: self.get_len_left_margin(),
        };
    }

    pub fn caret_position(&self) -> CaretPosition {
        self.caret_position
    }

    // ======================================== HELPER METHODS =======================================================
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
    fn set_size(&mut self, new_size: TerminalSize) {
        self.size = new_size;
    }

    /// Method to actually draw the component, must be implemented by each component
    fn draw(&mut self, origin_y: usize) -> Result<(), std::io::Error> {
        let line = self.construct_line();
        Terminal::print_row(origin_y, &line)
    }
}
