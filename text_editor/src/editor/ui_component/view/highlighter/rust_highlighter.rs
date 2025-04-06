use std::collections::HashMap;

use crate::editor::{annotated_string::{annotation::Annotation, annotation_type::AnnotationType}, line::{Line, LineIdx}};

use super::syntax_highlihter::SyntaxHighlighter;


pub struct RustSyntaxHighlighter {
    highlights: HashMap<LineIdx, Vec<Annotation>>,
}

impl RustSyntaxHighlighter {
    
    pub fn new() -> Self {
        RustSyntaxHighlighter { highlights: HashMap::new() }
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
    
}

impl SyntaxHighlighter for RustSyntaxHighlighter {
    fn highlight(&mut self, idx: LineIdx, line: &Line) {
        let mut result = Vec::new();
        
        Self::highlight_digits(line, &mut result);
        
        self.highlights.insert(idx, result);
    }
    
    fn get_annotations(&self, idx: LineIdx) -> Option<&Vec<Annotation>> {
        self.highlights.get(&idx)
    }
}