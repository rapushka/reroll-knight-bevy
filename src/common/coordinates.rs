use std::fmt::{Display, Formatter};

#[derive(Copy, Clone)]
pub struct Coordinates {
    pub column: i32,
    pub row: i32,
}

impl Coordinates {
    pub const ZERO: Self = Self { column: 0, row: 0 };

    pub fn new(column: i32, row: i32) -> Self { Self { column, row } }
}

impl Default for Coordinates {
    fn default() -> Self { Self::ZERO }
}

impl Display for Coordinates {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.column, self.row)
    }
}