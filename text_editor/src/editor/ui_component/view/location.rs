#[derive(Clone, Copy, Default, Debug)]
pub struct Location {
    pub grapheme_idx: usize,
    pub line_idx: usize,
}
