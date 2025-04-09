use super::{caret_position::CaretPosition, ui_component::view::highlighter::file_type::FileType};

#[derive(Default)]
pub struct DocumentStatus {
    pub caret_position: CaretPosition,
    pub file_name: Option<String>,
    pub number_of_lines: usize,
    pub is_modified: bool,
    pub file_type: FileType,
}

impl DocumentStatus {
    pub fn modified_indicator_to_string(&self) -> String {
        if self.is_modified {
            String::from("(modified)")
        } else {
            String::new()
        }
    }

    pub fn line_count_to_string(&self) -> String {
        format!("{} lines", self.number_of_lines)
    }

    pub fn position_indicator_to_string(&self) -> String {
        format!(
            "{}/{}",
            self.caret_position.row.saturating_add(1),
            self.number_of_lines
        )
    }
}
