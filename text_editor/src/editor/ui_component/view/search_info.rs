use crate::editor::line::Line;

use super::location::Location;

#[derive(Clone, Default)]
pub struct SearchInfo {
    pub prev_location: Location,
    pub search_query: Line,
}
