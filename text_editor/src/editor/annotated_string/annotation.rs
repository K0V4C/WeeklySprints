use crate::editor::line::ByteIdx;

use super::annotation_type::AnnotationType;

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
}
