pub type Col = usize;
pub type Row = usize;

#[derive(Copy, Clone, Default)]
pub struct CaretPosition {
    pub column: Col,
    pub row: Row,
}

impl CaretPosition {
    pub const fn saturating_sub(self, other: Self) -> Self {
        Self {
            column: self.column.saturating_sub(other.column),
            row: self.row.saturating_sub(other.row),
        }
    }
}
