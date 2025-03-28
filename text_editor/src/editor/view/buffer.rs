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
            self.data.push(Line::from(""));
        }

        if let Some(selected_line) = self.data.get_mut(location.line_index) {
            selected_line.add_character_to_line(chr, location.grapheme_index);
        };
    }

    pub fn delete_character_at(&mut self, location: Location) {
        // Safeguard, should not happen
        if location.line_index > self.data.len() {
            return;
        }

        // Backspace already transalted into delete so it only one case now
        // Delete when caret is at the end of the line

        if self.data.get_mut(location.line_index).is_none() {
            return;
        }

        let number_of_lines = self.get_number_of_lines();
        let selected_line = self.data.get_mut(location.line_index).unwrap();
        let line_length = selected_line.grapheme_count();

        // Deletion at non specific point
        if location.grapheme_index != line_length {
            selected_line.delete_character(location.grapheme_index);
        } else if location.grapheme_index == line_length && location.line_index.saturating_add(1) < number_of_lines {
            // Deletion at the end of the line
            let next_line = self.data.remove(location.line_index.saturating_add(1));
            // This is the same code for `selected_line` had to be written this way so the reference from before would be dropped
            let selected_line = self.data.get_mut(location.line_index).unwrap();
            selected_line.concat(next_line);
        }
    }

    pub fn insert_newline(&mut self, location: Location) {

        if location.line_index == self.get_number_of_lines() {
            self.data.push(Line::default());
            return;
        }

        if let Some(working_line) = self.data.get_mut(location.line_index) {
            let cut_off = working_line.split_off(location.grapheme_index);
            self.data.insert(location.line_index.saturating_add(1), cut_off);
        }
    }

    pub fn get_number_of_lines(&self) -> usize {
        self.data.len()
    }
}
