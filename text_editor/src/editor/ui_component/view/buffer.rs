use std::fs::OpenOptions;
use std::io::Write;

use crate::editor::line::Line;

use super::Location;

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

    // =========================================================== Search ===============================================================

    pub fn backward_find(
        &self,
        search_string: &str,
        current_location: Location,
    ) -> Option<Location> {
        let current_line = self.data.get(current_location.line_idx);

        let lines_after = self
            .data
            .iter()
            .enumerate()
            .filter(|(idx, _)| *idx >= current_location.line_idx)
            .rev();

        let lines_before = self
            .data
            .iter()
            .enumerate()
            .filter(|(idx, _)| *idx < current_location.line_idx)
            .rev();

        // Try to find on current line
        if let Some(line) = current_line {
            if let Some(graph_idx) =
                line.backward_find(search_string, current_location.grapheme_idx)
            {
                return Some(Location {
                    line_idx: current_location.line_idx,
                    grapheme_idx: graph_idx,
                });
            }
        }

        // Try to look backward
        if let Some(found) = Self::search_backward(search_string, lines_before) {
            return Some(found);
        }

        // Loop around and search
        if let Some(found) = Self::search_backward(search_string, lines_after) {
            return Some(found);
        }

        None
    }

    pub fn next_valid_search_location(&self, loc: Location, search_string: &str) -> Location {
        let value = self
            .data
            .get(loc.line_idx)
            .map_or(Line::from(search_string).grapheme_count(), |x| {
                x.get_next_match_idx(loc.grapheme_idx, search_string)
            });

        Location {
            grapheme_idx: loc.grapheme_idx.saturating_add(value),
            line_idx: loc.line_idx,
        }
    }

    pub fn forward_find(
        &self,
        search_string: &str,
        current_location: Location,
    ) -> Option<Location> {
        let current_line = self.data.get(current_location.line_idx);

        let lines_after = self
            .data
            .iter()
            .enumerate()
            .filter(|(idx, _)| *idx > current_location.line_idx);

        let lines_before = self
            .data
            .iter()
            .enumerate()
            .filter(|(idx, _)| *idx <= current_location.line_idx);

        // Try to find on current line
        if let Some(line) = current_line {
            if let Some(graph_idx) = line.forward_find(search_string, current_location.grapheme_idx)
            {
                return Some(Location {
                    line_idx: current_location.line_idx,
                    grapheme_idx: graph_idx,
                });
            }
        }

        // Try to look forward
        if let Some(found) = Self::search_forward(search_string, lines_after) {
            return Some(found);
        }

        // Loop around and search
        if let Some(found) = Self::search_forward(search_string, lines_before) {
            return Some(found);
        }

        None
    }

    fn search_forward<'a, I>(search_string: &'a str, line_pile: I) -> Option<Location>
    where
        I: Iterator<Item = (usize, &'a Line)>,
    {
        // Try to look forward
        for (line_idx, line) in line_pile {
            if let Some(graph_idx) = line.forward_find(search_string, 0) {
                return Some(Location {
                    line_idx,
                    grapheme_idx: graph_idx,
                });
            }
        }
        None
    }

    fn search_backward<'a, I>(search_string: &'a str, line_pile: I) -> Option<Location>
    where
        I: Iterator<Item = (usize, &'a Line)>,
    {
        // Try to look forward
        for (line_idx, line) in line_pile {
            if let Some(graph_idx) =
                line.backward_find(search_string, line.grapheme_count().saturating_sub(1))
            {
                return Some(Location {
                    line_idx,
                    grapheme_idx: graph_idx,
                });
            }
        }
        None
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
        if location.line_idx > self.data.len() {
            return;
        }

        // TODO: maybe should fix this and move this piece of code inside Some block
        if location.line_idx == self.data.len() {
            self.data.push(Line::from(""));
        }

        if let Some(selected_line) = self.data.get_mut(location.line_idx) {
            selected_line.add_character_to_line(chr, location.grapheme_idx);
            self.is_modified = true;
        };
    }

    pub fn delete_character_at(&mut self, location: Location) {
        // Safeguard, should not happen
        if location.line_idx > self.data.len() {
            return;
        }

        // Backspace already transalted into delete so it only one case now
        // Delete when caret is at the end of the line

        if self.data.get_mut(location.line_idx).is_none() {
            return;
        }

        let number_of_lines = self.get_number_of_lines();
        let selected_line = self.data.get_mut(location.line_idx).unwrap();
        let line_length = selected_line.grapheme_count();

        // Deletion at non specific point
        if location.grapheme_idx != line_length {
            selected_line.delete_character(location.grapheme_idx);
            self.is_modified = true;
        } else if location.grapheme_idx == line_length
            && location.line_idx.saturating_add(1) < number_of_lines
        {
            // Deletion at the end of the line
            let next_line = self.data.remove(location.line_idx.saturating_add(1));
            // This is the same code for `selected_line` had to be written this way so the reference from before would be dropped
            let selected_line = self.data.get_mut(location.line_idx).unwrap();
            selected_line.concat(&next_line);
            self.is_modified = true;
        }
    }

    pub fn insert_newline(&mut self, location: Location) {
        if location.line_idx == self.get_number_of_lines() {
            self.data.push(Line::default());
            return;
        }

        if let Some(working_line) = self.data.get_mut(location.line_idx) {
            let cut_off = working_line.split_off(location.grapheme_idx);
            self.data
                .insert(location.line_idx.saturating_add(1), cut_off);
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
