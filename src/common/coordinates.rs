use std::fmt::{Display, Formatter};

#[derive(Copy, Clone)]
pub struct Coordinates {
    pub column: i32,
    pub row: i32,
}

impl Coordinates {
    pub const ZERO: Self = Self { column: 0, row: 0 };

    pub const fn new(column: i32, row: i32) -> Self { Self { column, row } }
}

impl Default for Coordinates {
    fn default() -> Self { Self::ZERO }
}

impl Display for Coordinates {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.column, self.row)
    }
}

impl IntoIterator for Coordinates {
    type Item = (i32, i32);
    type IntoIter = Box<dyn Iterator<Item = (i32, i32)>>;

    fn into_iter(self) -> Self::IntoIter {
        let column_range = 0..=self.column;
        let row_range = 0..=self.row;
        let iter = column_range.flat_map(move |column| {
            row_range.clone().map(move |row| (column, row))
        });
        Box::new(iter)
    }
}