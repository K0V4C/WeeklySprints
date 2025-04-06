use std::ops::Range;

use annotated_string_iterator::AnnotatedStringIterator;
use annotated_string_part::AnnotatedStringPart;
use annotation::Annotation;

mod annotated_string_iterator;
mod annotated_string_part;
pub mod annotation;
pub mod annotation_type;

pub struct AnnotatedString {
    pub string: String,
    pub annotations: Vec<Annotation>,
}

impl AnnotatedString {
    pub fn add_annotation(&mut self, new_annotation: Annotation) {
        self.annotations.push(new_annotation);
    }

    // TODO: fix so it works with graphemes
    pub fn crop(&mut self, range: Range<usize>) {
        if range.start >= range.end {
            return;
        }

        if range.start > self.string.len() || range.end > self.string.len() {
            return;
        }

        let mut new_annotations: Vec<Annotation> = Vec::new();
        let cropped_string = &self.string[range.clone()];

        for annotation in &self.annotations {
            if (range.start < annotation.start_byte && range.end < annotation.start_byte)
                || (range.end > annotation.start_byte && range.end > annotation.end_byte)
            {
                continue;
            }

            // RS AS RE AE

            if range.start < annotation.start_byte
                && range.start < annotation.end_byte
                && range.end > annotation.start_byte
                && range.end < annotation.end_byte
            {
                new_annotations.push(Annotation {
                    start_byte: annotation.start_byte,
                    end_byte: range.end,
                    annotation_type: annotation.annotation_type,
                });
            }

            // AS RS AE RE

            if range.start < annotation.end_byte
                && range.start > annotation.start_byte
                && range.end > annotation.end_byte
                && range.end > annotation.start_byte
            {
                new_annotations.push(Annotation {
                    start_byte: range.start,
                    end_byte: annotation.end_byte,
                    annotation_type: annotation.annotation_type,
                });
            }

            // AS RS RE AE

            if range.start > annotation.start_byte
                && range.start < annotation.end_byte
                && range.end < annotation.end_byte
                && range.end > annotation.start_byte
            {
                new_annotations.push(Annotation {
                    start_byte: range.start,
                    end_byte: range.end,
                    annotation_type: annotation.annotation_type,
                });
            }

            // RS AS AE RE

            if range.start < annotation.start_byte
                && range.start < annotation.end_byte
                && range.end > annotation.end_byte
                && range.end > annotation.start_byte
            {
                new_annotations.push(Annotation {
                    start_byte: annotation.start_byte,
                    end_byte: annotation.end_byte,
                    annotation_type: annotation.annotation_type,
                });
            }
        }

        self.annotations = new_annotations;
        self.string = cropped_string.to_owned();
    }
}

impl<'a> IntoIterator for &'a AnnotatedString {
    type Item = AnnotatedStringPart<'a>;
    type IntoIter = AnnotatedStringIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        AnnotatedStringIterator {
            annotated_string: self,
            current_idx: 0,
        }
    }
}
