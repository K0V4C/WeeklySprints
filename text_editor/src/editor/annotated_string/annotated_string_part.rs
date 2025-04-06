use super::annotation_type::AnnotationType;

#[derive(Debug)]
pub struct AnnotatedStringPart<'a> {
    pub string: &'a str,
    pub annotaion_type: Option<AnnotationType>,
}
