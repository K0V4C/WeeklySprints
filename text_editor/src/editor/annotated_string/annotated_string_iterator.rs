use super::{AnnotatedString, annotated_string_part::AnnotatedStringPart};

pub struct AnnotatedStringIterator<'a> {
    pub annotated_string: &'a AnnotatedString,
    pub current_idx: usize,
}

impl<'a> Iterator for AnnotatedStringIterator<'a> {
    type Item = AnnotatedStringPart<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_idx >= self.annotated_string.string.len() {
            return None;
        }

        if let Some(annotation) = self
            .annotated_string
            .annotations
            .iter()
            .filter(|annotation| {
                annotation.start_byte <= self.current_idx && annotation.end_byte > self.current_idx
            })
            .last()
        {
            let end_idx = std::cmp::min(annotation.end_byte, self.annotated_string.string.len());
            let start_idx = self.current_idx;
            self.current_idx = end_idx;
            return Some(AnnotatedStringPart {
                string: &self.annotated_string.string[start_idx..end_idx],
                annotaion_type: Some(annotation.annotation_type),
            });
        };

        let mut end_idx = self.annotated_string.string.len();
        for annotation in &self.annotated_string.annotations {
            if annotation.start_byte > self.current_idx && annotation.start_byte < end_idx {
                end_idx = annotation.start_byte;
            }
        }

        let start_idx = self.current_idx;
        self.current_idx = end_idx;

        Some(AnnotatedStringPart {
            string: &self.annotated_string.string[start_idx..end_idx],
            annotaion_type: None,
        })
    }
}
