use crate::editor::terminal::CaretPosition;

#[derive(Clone, Copy, Default, Debug)]
pub struct Location {
    pub x: usize,
    pub y: usize,
}

impl From<Location> for CaretPosition {

    fn from(value: Location) -> Self {
        Self {
            column: value.x,
            row: value.y
        }
    }

}

impl Location {
    pub const fn substract(&self, other: &Self) -> Self {
        Self{
            x: self.x.saturating_sub(other.x),
            y: self.y.saturating_sub(other.y)
        }
    }
}
