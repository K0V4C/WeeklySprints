use std::collections::HashMap;

use crate::editor::{annotated_string::{annotation::Annotation, annotation_type::AnnotationType}, line::Line, ui_component::view::location::Location};

use super::syntax_highlihter::SyntaxHighlighter;


pub struct SearchHighlighter {    
    matched_word: Option<String>,
    selected_match: Option<Location>,
    highlights: HashMap<usize, Vec<Annotation>>,
}

impl SearchHighlighter {    
    
    pub fn new(matched_word: Option<String>, selected_match: Option<Location>) -> Self {
        SearchHighlighter { matched_word, selected_match, highlights: HashMap::new() }
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
}

impl SyntaxHighlighter for SearchHighlighter {
    fn highlight(&mut self, idx: crate::editor::line::LineIdx, line: &crate::editor::line::Line) {
        let mut result = Vec::new();
        self.highlight_matched_words(line, &mut result);
        if let Some(selected_match) = self.selected_match {
            if selected_match.line_idx == idx {
                self.highlight_selected_match(&mut result);
            }
        }
        self.highlights.insert(idx, result);
    }
    
    fn get_annotations(&self, idx: crate::editor::line::LineIdx) -> Option<&Vec<Annotation>> {
        self.highlights.get(&idx)
    }
}