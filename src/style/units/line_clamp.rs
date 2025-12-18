use std::fmt;

use crate::style::style_definition::MergeFrom;

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub enum LineClamp {
    Lines(u16),
    Disable,
}

impl fmt::Display for LineClamp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LineClamp::Lines(n) => {
                f.write_str("overflow: hidden;")?;
                f.write_str("display: -webkit-box;")?;
                f.write_str("-webkit-box-orient: vertical;")?;
                write!(f, "-webkit-line-clamp: {n};")?;
            }
            LineClamp::Disable => {
                f.write_str("overflow: visible;")?;
                f.write_str("display: block;")?;
                f.write_str("-webkit-box-orient: horizontal;")?;
                f.write_str("-webkit-line-clamp: none;")?;
            }
        }
        Ok(())
    }
}

impl MergeFrom for LineClamp {
    fn merge_from(&mut self, other: Self) {
        *self = other;
    }
}
