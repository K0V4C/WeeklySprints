use std::fmt::Display;

use crate::editor::line::ByteIdx;

use super::annotation_type::AnnotationType;

#[derive(Clone, Copy, Debug)]
pub struct Annotation {
    pub start_byte: ByteIdx,
    pub end_byte: ByteIdx,
    pub annotation_type: AnnotationType,
}

impl Annotation {
    pub fn new(start_byte: ByteIdx, end_byte: ByteIdx, annotation_type: AnnotationType) -> Self {
        Self {
            start_byte,
            end_byte,
            annotation_type,
        }
    }

    pub fn shift(&mut self, offset: ByteIdx) {
        self.start_byte = self.start_byte.saturating_add(offset);
        self.end_byte = self.end_byte.saturating_add(offset);
    }
}

impl Display for Annotation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} \n", self.start_byte, self.end_byte)
    }
}
