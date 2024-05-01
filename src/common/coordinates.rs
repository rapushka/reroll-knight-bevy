#[derive(Copy, Clone)]
pub struct Coordinates {
    pub column: i32,
    pub row: i32,
}

impl Coordinates {
    pub const ZERO: Self = Self { column: 0, row: 0 };
}

impl Default for Coordinates {
    fn default() -> Self { Self::ZERO }
}