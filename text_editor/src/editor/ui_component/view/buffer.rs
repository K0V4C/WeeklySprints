use std::fs::OpenOptions;
use std::io::Write;

use super::{Location, line::Line};

#[derive(Default)]
pub struct Buffer {
    pub data: Vec<Line>,
    file_name: Option<String>,
    is_modified: bool,
}

impl Buffer {
    // ==================================================== Simple Manipulation Methods ================================================

    pub fn clear(&mut self) {
        self.data.clear();
    }

    /// Does not update modfied status
    pub fn push(&mut self, string: &str) {
        self.data.push(Line::from(string));
    }

    pub fn set_file(&mut self, file_name: &str) {
        self.file_name = Some(file_name.to_string());
    }

    // ============================================================= Getters ==========================================================

    pub fn get_file_name(&self) -> Option<String> {
        self.file_name.clone()
    }

    pub fn is_modified(&self) -> bool {
        self.is_modified
    }

    pub fn is_file_given(&self) -> bool {
        self.file_name.is_some()
    }

    pub fn get_number_of_lines(&self) -> usize {
        self.data.len()
    }

    // ====================================================== Buffer Edditing ===================================================

    pub fn add_character_at(&mut self, chr: char, location: Location) {
        if location.line_index > self.data.len() {
            return;
        }

        // TODO: maybe should fix this and move this piece of code inside Some block
        if location.line_index == self.data.len() {
            self.data.push(Line::from(""));
        }

        if let Some(selected_line) = self.data.get_mut(location.line_index) {
            selected_line.add_character_to_line(chr, location.grapheme_index);
            self.is_modified = true;
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
            self.is_modified = true;
        } else if location.grapheme_index == line_length
            && location.line_index.saturating_add(1) < number_of_lines
        {
            // Deletion at the end of the line
            let next_line = self.data.remove(location.line_index.saturating_add(1));
            // This is the same code for `selected_line` had to be written this way so the reference from before would be dropped
            let selected_line = self.data.get_mut(location.line_index).unwrap();
            selected_line.concat(next_line);
            self.is_modified = true;
        }
    }

    pub fn insert_newline(&mut self, location: Location) {
        if location.line_index == self.get_number_of_lines() {
            self.data.push(Line::default());
            return;
        }

        if let Some(working_line) = self.data.get_mut(location.line_index) {
            let cut_off = working_line.split_off(location.grapheme_index);
            self.data
                .insert(location.line_index.saturating_add(1), cut_off);
        }

        self.is_modified = true;
    }

    // =================================================== Loading/Saving File ======================================================

    pub fn load(&mut self, file_name: &str) {
        if let Ok(context) = std::fs::read_to_string(file_name) {
            self.clear();
            self.file_name = Some(file_name.to_string());

            for line in context.lines() {
                self.data.push(Line::from(line));
            }
        }
    }

    pub fn save(&mut self) -> Result<(), std::io::Error> {
        if let Some(file_name) = self.file_name.clone() {
            let mut file = OpenOptions::new()
                .write(true)
                .create(true)
                .truncate(true)
                .open(&file_name)?;

            for line in &self.data {
                let string = line.to_string();

                writeln!(file, "{string}")?;
            }

            self.is_modified = false;
        }

        Ok(())
    }
}
