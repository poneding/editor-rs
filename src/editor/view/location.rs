use super::Position;

#[derive(Default, Clone, Copy)]
pub struct Location {
    pub x: usize,
    pub y: usize,
}

impl From<Location> for Position {
    fn from(location: Location) -> Self {
        Self {
            col: location.x,
            row: location.y,
        }
    }
}

impl Location {
    pub const fn subtract(&self, rhs: &Self) -> Self {
        Self {
            x: self.x.saturating_sub(rhs.x),
            y: self.y.saturating_sub(rhs.y),
        }
    }
}
