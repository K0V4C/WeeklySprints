use std::collections::HashMap;

use crate::editor::{
    annotated_string::{annotation::Annotation, annotation_type::AnnotationType},
    line::Line,
};

use super::location::Location;

pub struct Highlighter {
    matched_word: Option<String>,
    selected_match: Option<Location>,
    highlights: HashMap<usize, Vec<Annotation>>,
}

impl Highlighter {
    pub fn new(matched_word: Option<String>, selected_match: Option<Location>) -> Self {
        Highlighter {
            matched_word,
            selected_match,
            highlights: HashMap::new(),
        }
    }

    pub fn get_annotations(&self, line_idx: usize) -> Option<&Vec<Annotation>> {
        self.highlights.get(&line_idx)
    }

    fn highlight_digits(line: &Line, result: &mut Vec<Annotation>) {
        line.to_string().chars().enumerate().for_each(|(idx, ch)| {
            if ch.is_ascii_digit() {
                result.push(Annotation {
                    annotation_type: AnnotationType::Digit,
                    start_byte: idx,
                    end_byte: idx.saturating_add(1),
                });
            }
        });
    }

    fn highlight_matched_words(&self, line: &Line, result: &mut Vec<Annotation>) {
        if let Some(matched_word) = &self.matched_word {
            if matched_word.is_empty() {
                return;
            }
            line.find_all(&matched_word, 0..line.len())
                .iter()
                .for_each(|(start, _)| {
                    result.push(Annotation {
                        annotation_type: AnnotationType::Match,
                        start_byte: *start,
                        end_byte: start.saturating_add(matched_word.len()),
                    });
                });
        }
    }

    fn highlight_selected_match(&self, result: &mut Vec<Annotation>) {
        if let Some(selected_match) = self.selected_match {
            if let Some(matched_word) = &self.matched_word {
                if matched_word.is_empty() {
                    return;
                }
                let start = selected_match.grapheme_idx;
                result.push(Annotation {
                    annotation_type: AnnotationType::SelectedMatch,
                    start_byte: start,
                    end_byte: start.saturating_add(matched_word.len()),
                });
            }
        }
    }

    pub fn highlight(&mut self, idx: usize, line: &Line) {
        let mut result = Vec::new();
        
        Self::highlight_digits(line, &mut result);
        self.highlight_matched_words(line, &mut result);
        
        if let Some(selected_match) = self.selected_match {
            if selected_match.line_idx == idx {
                self.highlight_selected_match(&mut result);
            }
        }

        let relative_line_idx = idx;
        self.highlights.insert(relative_line_idx, result);
    }
}
