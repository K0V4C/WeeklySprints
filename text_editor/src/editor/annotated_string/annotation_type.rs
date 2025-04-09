#[derive(Clone, Copy, Debug)]
pub enum AnnotationType {
    Match,
    SelectedMatch,
    Number,
    Type,
    KeyWord,
    KnownValue,
    Char,
    Lifetime,
    Comment,
    String,
}
