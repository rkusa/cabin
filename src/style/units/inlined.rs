use std::fmt::{self, Display};

use crate::style::property_display::PropertyDisplay;
use crate::style::style_definition::MergeFrom;

#[derive(Clone, Hash, PartialEq, Eq)]
pub struct Inlined<T> {
    pub block_start: Option<T>,
    pub inline_end: Option<T>,
    pub block_end: Option<T>,
    pub inline_start: Option<T>,
}

impl<T: Copy> Inlined<T> {
    pub fn set(&mut self, value: T) {
        self.block_start = Some(value);
        self.inline_end = Some(value);
        self.block_end = Some(value);
        self.inline_start = Some(value);
    }

    pub fn set_x(&mut self, value: T) {
        self.inline_end = Some(value);
        self.inline_start = Some(value);
    }

    pub fn set_y(&mut self, value: T) {
        self.block_start = Some(value);
        self.block_end = Some(value);
    }

    pub fn set_block_start(&mut self, value: T) {
        self.block_start = Some(value);
    }

    pub fn set_inline_end(&mut self, value: T) {
        self.inline_end = Some(value);
    }

    pub fn set_block_end(&mut self, value: T) {
        self.block_end = Some(value);
    }

    pub fn set_inline_start(&mut self, value: T) {
        self.inline_start = Some(value);
    }

    pub fn to_reversed(&self, x: bool, y: bool) -> Self {
        Self {
            block_start: if y { self.block_end } else { self.block_start },
            inline_end: if x {
                self.inline_start
            } else {
                self.inline_end
            },
            block_end: if y { self.block_start } else { self.block_end },
            inline_start: if x {
                self.inline_end
            } else {
                self.inline_start
            },
        }
    }
}

impl<T: PartialEq + Display> PropertyDisplay for Inlined<T> {
    fn fmt_property(&self, name: &str, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (name, suffix) = if let Some((prefix, _)) = name.split_once('-') {
            (prefix, &name[prefix.len()..])
        } else {
            (name, "")
        };
        if let Some((start, end)) = self.block_start.as_ref().zip(self.block_end.as_ref())
            && start == end
        {
            writeln!(f, "{name}-block{suffix}: {start};")?;
        } else {
            if let Some(start) = &self.block_start {
                writeln!(f, "{name}-block-start{suffix}: {start};")?;
            }
            if let Some(end) = &self.block_end {
                writeln!(f, "{name}-block-end{suffix}: {end};")?;
            }
        }
        if let Some((start, end)) = self.inline_start.as_ref().zip(self.inline_end.as_ref())
            && start == end
        {
            writeln!(f, "{name}-inline{suffix}: {start};")?;
        } else {
            if let Some(start) = &self.inline_start {
                writeln!(f, "{name}-inline-start{suffix}: {start};")?;
            }
            if let Some(end) = &self.inline_end {
                writeln!(f, "{name}-inline-end{suffix}: {end};")?;
            }
        }
        Ok(())
    }
}

impl<T> Default for Inlined<T> {
    fn default() -> Self {
        Self {
            block_start: None,
            inline_end: None,
            block_end: None,
            inline_start: None,
        }
    }
}

impl<T: MergeFrom> MergeFrom for Inlined<T> {
    fn merge_from(&mut self, other: Self) {
        let Self {
            block_start,
            inline_end,
            block_end,
            inline_start,
        } = other;
        self.block_start.merge_from(block_start);
        self.inline_end.merge_from(inline_end);
        self.block_end.merge_from(block_end);
        self.inline_start.merge_from(inline_start);
    }
}
