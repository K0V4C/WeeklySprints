use super::location::Location;


#[derive(Clone, Copy, Default, Debug)]
pub struct SearchInfo {
    pub prev_location: Location,
}