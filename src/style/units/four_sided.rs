use std::fmt::{self, Display};

use crate::style::property_display::PropertyDisplay;
use crate::style::style_definition::MergeFrom;

#[derive(Clone, Hash, PartialEq, Eq)]
pub struct FourSided<T> {
    pub top: Option<T>,
    pub right: Option<T>,
    pub bottom: Option<T>,
    pub left: Option<T>,
}

impl<T: Copy> FourSided<T> {
    pub fn set(&mut self, value: T) {
        self.top = Some(value);
        self.right = Some(value);
        self.bottom = Some(value);
        self.left = Some(value);
    }

    pub fn set_x(&mut self, value: T) {
        self.right = Some(value);
        self.left = Some(value);
    }

    pub fn set_y(&mut self, value: T) {
        self.top = Some(value);
        self.bottom = Some(value);
    }

    pub fn set_top(&mut self, value: T) {
        self.top = Some(value);
    }

    pub fn set_right(&mut self, value: T) {
        self.right = Some(value);
    }

    pub fn set_bottom(&mut self, value: T) {
        self.bottom = Some(value);
    }

    pub fn set_left(&mut self, value: T) {
        self.left = Some(value);
    }
}

impl<T: PartialEq + Display> PropertyDisplay for FourSided<T> {
    fn fmt_property(&self, name: &str, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match (&self.top, &self.right, &self.bottom, &self.left) {
            (Some(top), Some(right), Some(bottom), Some(left))
                if top == right && right == bottom && bottom == left =>
            {
                writeln!(f, "{name}: {top};")
            }
            (Some(top), Some(right), Some(bottom), Some(left))
                if top == bottom && left == right =>
            {
                // top and bottom | left and right
                writeln!(f, "{name}: {top} {right};")
            }
            (Some(top), Some(right), Some(bottom), Some(left)) if left == right => {
                // top | left and right | bottom
                writeln!(f, "{name}: {top} {right} {bottom};")
            }
            (Some(top), Some(right), Some(bottom), Some(left)) => {
                writeln!(f, "{name}: {top} {right} {bottom} {left};")
            }
            (top, right, bottom, left) => {
                // FIXME: generalize special handling?
                if name == "inset" {
                    if let Some(top) = top {
                        writeln!(f, "top: {top};")?
                    }
                    if let Some(right) = right {
                        writeln!(f, "right: {right};")?
                    }
                    if let Some(bottom) = bottom {
                        writeln!(f, "bottom: {bottom};")?
                    }
                    if let Some(left) = left {
                        writeln!(f, "left: {left};")?
                    }
                } else {
                    let (name, suffix) = if let Some((prefix, _)) = name.split_once('-') {
                        (prefix, &name[prefix.len()..])
                    } else {
                        (name, "")
                    };
                    if let Some(top) = top {
                        writeln!(f, "{name}-top{suffix}: {top};")?
                    }
                    if let Some(right) = right {
                        writeln!(f, "{name}-right{suffix}: {right};")?
                    }
                    if let Some(bottom) = bottom {
                        writeln!(f, "{name}-bottom{suffix}: {bottom};")?
                    }
                    if let Some(left) = left {
                        writeln!(f, "{name}-left{suffix}: {left};")?
                    }
                }
                Ok(())
            }
        }
    }
}

impl<T> Default for FourSided<T> {
    fn default() -> Self {
        Self {
            top: None,
            right: None,
            bottom: None,
            left: None,
        }
    }
}

impl<T: MergeFrom> MergeFrom for FourSided<T> {
    fn merge_from(&mut self, other: Self) {
        let Self {
            top,
            right,
            bottom,
            left,
        } = other;
        self.top.merge_from(top);
        self.right.merge_from(right);
        self.bottom.merge_from(bottom);
        self.left.merge_from(left);
    }
}
