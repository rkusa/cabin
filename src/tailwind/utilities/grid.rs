//! Set the line names and track sizing functions of the grid columns (`grid-template-columns`).
//!
//! <https://developer.mozilla.org/en-US/docs/Web/CSS/grid-template-columns>

pub mod flow;

use std::fmt;

use crate::tailwind::Property;

const GRID_TEMPLATE_COLUMNS: &str = "grid-template-columns";
const GRID_TEMPLATE_ROWS: &str = "grid-template-rows";

/// ```css
/// grid-template-columns: repeat({n}, minmax(0, 1fr));
/// ```
pub fn cols(n: u16) -> Property<GridTemplate> {
    Property(GRID_TEMPLATE_COLUMNS, GridTemplate::Count(n))
}

pub mod cols {
    use super::{GRID_TEMPLATE_COLUMNS, GridTemplate};
    use crate::tailwind::Property;

    /// ```css
    /// grid-template-columns: none;
    /// ```
    pub const NONE: Property = Property(GRID_TEMPLATE_COLUMNS, "none");

    /// ```css
    /// grid-template-columns:{template};
    /// ```
    pub fn custom(template: &'static str) -> Property<GridTemplate> {
        Property(GRID_TEMPLATE_COLUMNS, GridTemplate::Custom(template))
    }
}

/// ```css
/// grid-template-rows: repeat({n}, minmax(0, 1fr));
/// ```
pub fn rows(n: u16) -> Property<GridTemplate> {
    Property(GRID_TEMPLATE_ROWS, GridTemplate::Count(n))
}

pub mod rows {
    use super::{GRID_TEMPLATE_ROWS, GridTemplate};
    use crate::tailwind::Property;

    /// ```css
    /// grid-template-rows: none;
    /// ```
    pub const NONE: Property = Property(GRID_TEMPLATE_ROWS, "none");

    /// ```css
    /// grid-template-rows:{template};
    /// ```
    pub fn custom(template: &'static str) -> Property<GridTemplate> {
        Property(GRID_TEMPLATE_ROWS, GridTemplate::Custom(template))
    }
}

pub enum GridTemplate {
    Count(u16),
    Custom(&'static str),
}

impl fmt::Display for GridTemplate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GridTemplate::Count(n) => write!(f, "repeat({n}, minmax(0, 1fr))"),
            GridTemplate::Custom(t) => f.write_str(t),
        }
    }
}
