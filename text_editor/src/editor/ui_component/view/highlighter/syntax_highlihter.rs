use crate::editor::{annotated_string::annotation::Annotation, line::{Line, LineIdx}};


pub trait SyntaxHighlighter {
    fn highlight(&mut self, idx: LineIdx, line: &Line);
    fn get_annotations(&self, idx: LineIdx) -> Option<&Vec<Annotation>>;   
}