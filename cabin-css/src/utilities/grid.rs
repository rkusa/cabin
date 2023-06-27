//! Set the line names and track sizing functions of the grid columns (`grid-template-columns`).
//!
//! <https://developer.mozilla.org/en-US/docs/Web/CSS/grid-template-columns>

use std::fmt;

use crate::Property;

const GRID_TEMPLATE_COLUMNS: &str = "grid-template-columns";

/// `grid-template-columns: repeat({n}, minmax(0, 1fr));`
pub fn cols(n: u16) -> Property<GridTemplateColumns> {
    Property(GRID_TEMPLATE_COLUMNS, GridTemplateColumns::Count(n))
}

pub mod cols {
    use crate::Property;

    use super::{GridTemplateColumns, GRID_TEMPLATE_COLUMNS};

    /// `grid-template-columns: none;`
    pub const NONE: Property = Property(GRID_TEMPLATE_COLUMNS, "none");

    /// `grid-template-columns: repeat({n}, minmax(0, 1fr));`
    pub fn custom(template: &'static str) -> Property<GridTemplateColumns> {
        Property(GRID_TEMPLATE_COLUMNS, GridTemplateColumns::Custom(template))
    }
}

pub enum GridTemplateColumns {
    Count(u16),
    Custom(&'static str),
}

impl fmt::Display for GridTemplateColumns {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GridTemplateColumns::Count(n) => write!(f, "repeat({n}, minmax(0, 1fr))"),
            GridTemplateColumns::Custom(t) => f.write_str(t),
        }
    }
}
