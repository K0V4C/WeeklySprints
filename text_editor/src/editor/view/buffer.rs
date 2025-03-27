use super::{Location, line::Line};

#[derive(Default)]
pub struct Buffer {
    pub data: Vec<Line>,
}

impl Buffer {
    pub fn clear(&mut self) {
        self.data.clear();
    }

    pub fn push(&mut self, string: &str) {
        self.data.push(Line::from(string));
    }

    pub fn add_character_at(&mut self, chr: char, location: Location) {
        if location.line_index > self.data.len() {
            return;
        }

        if location.line_index == self.data.len() {
            self.data.push(Line::from(" "));
        }

        if let Some(selected_line) = self.data.get_mut(location.line_index) {
            selected_line.add_character_to_line(chr, location.grapheme_index);
        };
    }

    pub fn delete_character_at(&mut self, location: Location) {
        if location.line_index > self.data.len() {
            return;
        }

        // TODO: Add  deletion of lines themselves and concating

        if let Some(selected_line) = self.data.get_mut(location.line_index) {
            selected_line.delete_character(location.grapheme_index);
        };
    }

    pub fn get_number_of_lines(&self) -> usize {
        self.data.len()
    }
}
