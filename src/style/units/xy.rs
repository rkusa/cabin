use std::fmt::{self, Display};

use crate::style::property_display::PropertyDisplay;
use crate::style::style_definition::MergeFrom;

#[derive(Clone, Hash, PartialEq, Eq)]
pub struct Xy<T> {
    pub x: Option<T>,
    pub y: Option<T>,
}

impl<T: Copy> Xy<T> {
    pub fn set(&mut self, value: T) {
        self.x = Some(value);
        self.y = Some(value);
    }

    pub fn set_x(&mut self, value: T) {
        self.x = Some(value);
    }

    pub fn set_y(&mut self, value: T) {
        self.y = Some(value);
    }
}

impl<T: PartialEq + Display> PropertyDisplay for Xy<T> {
    fn fmt_property(&self, name: &str, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some((x, y)) = self.x.as_ref().zip(self.y.as_ref())
            && x == y
        {
            writeln!(f, "{name}: {x};")?;
        } else {
            if let Some(x) = &self.x {
                writeln!(f, "{name}-x: {x};")?;
            }
            if let Some(y) = &self.y {
                writeln!(f, "{name}-y: {y};")?;
            }
        }
        Ok(())
    }
}

impl<T> Default for Xy<T> {
    fn default() -> Self {
        Self { x: None, y: None }
    }
}

impl<T: MergeFrom> MergeFrom for Xy<T> {
    fn merge_from(&mut self, other: Self) {
        let Self { x, y } = other;
        self.x.merge_from(x);
        self.y.merge_from(y);
    }
}
