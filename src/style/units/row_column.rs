use std::fmt::{self, Display};

use crate::style::property_display::PropertyDisplay;

#[derive(Clone)]
pub struct RowColumn<T> {
    pub row: Option<T>,
    pub column: Option<T>,
}

impl<T: Copy> RowColumn<T> {
    pub fn set(&mut self, value: T) {
        self.row = Some(value);
        self.column = Some(value);
    }

    pub fn set_row(&mut self, value: T) {
        self.row = Some(value);
    }

    pub fn set_column(&mut self, value: T) {
        self.column = Some(value);
    }
}

impl<T: PartialEq + Display> PropertyDisplay for RowColumn<T> {
    fn fmt_property(&self, name: &str, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some((row, column)) = self.row.as_ref().zip(self.column.as_ref())
            && row == column
        {
            writeln!(f, "{name}: {row};")?;
        } else {
            if let Some(row) = &self.row {
                writeln!(f, "row-{name}: {row};")?;
            }
            if let Some(column) = &self.column {
                writeln!(f, "column-{name}: {column};")?;
            }
        }
        Ok(())
    }
}

impl<T> Default for RowColumn<T> {
    fn default() -> Self {
        Self {
            row: None,
            column: None,
        }
    }
}
