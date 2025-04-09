pub mod file_type;
mod rust_highlighter;
mod search_highlighter;
mod syntax_highlihter;

use file_type::FileType;
use rust_highlighter::RustSyntaxHighlighter;
use search_highlighter::SearchHighlighter;
use syntax_highlihter::SyntaxHighlighter;

use crate::editor::{annotated_string::annotation::Annotation, line::Line};

use super::location::Location;

pub struct Highlighter {
    syntax_highlighter: Option<Box<dyn SyntaxHighlighter>>,
    search_highlighter: Option<Box<dyn SyntaxHighlighter>>,
}

impl Highlighter {
    pub fn new(
        matched_word: Option<String>,
        selected_match: Option<Location>,
        file_type: Option<FileType>,
    ) -> Self {
        Highlighter {
            syntax_highlighter: Self::create_syntax_highlighter(file_type),
            search_highlighter: Self::create_search_highlighter(matched_word, selected_match),
        }
    }

    fn create_syntax_highlighter(
        file_type: Option<FileType>,
    ) -> Option<Box<dyn SyntaxHighlighter>> {
        if let Some(file_type) = file_type {
            match file_type {
                FileType::Rust => return Some(Box::new(RustSyntaxHighlighter::new())),
                _ => return None,
            }
        }
        None
    }

    fn create_search_highlighter(
        matched_word: Option<String>,
        selected_match: Option<Location>,
    ) -> Option<Box<dyn SyntaxHighlighter>> {
        Some(Box::new(SearchHighlighter::new(
            matched_word,
            selected_match,
        )))
    }

    pub fn get_annotations(&self, line_idx: usize) -> Vec<Annotation> {
        let mut result = Vec::new();

        if let Some(syntax) = &self.syntax_highlighter {
            if let Some(annotations) = syntax.get_annotations(line_idx) {
                result.extend(annotations.iter().copied());
            }
        }

        if let Some(search) = &self.search_highlighter {
            if let Some(annotations) = search.get_annotations(line_idx) {
                result.extend(annotations.iter().copied());
            }
        }

        result
    }

    pub fn highlight(&mut self, idx: usize, line: &Line) {
        if let Some(just_syntax) = &mut self.syntax_highlighter {
            just_syntax.as_mut().highlight(idx, line);
        }

        if let Some(search) = &mut self.search_highlighter {
            search.as_mut().highlight(idx, line);
        }
    }
}
