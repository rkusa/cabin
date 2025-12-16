use std::fmt::{self, Display};

use crate::style::property_display::PropertyDisplay;

#[derive(Clone)]
pub struct Corners<T> {
    pub top_left: Option<T>,
    pub top_right: Option<T>,
    pub bottom_right: Option<T>,
    pub bottom_left: Option<T>,
}

impl<T: Copy> Corners<T> {
    pub fn set(&mut self, value: T) {
        self.top_left = Some(value);
        self.top_right = Some(value);
        self.bottom_right = Some(value);
        self.bottom_left = Some(value);
    }

    pub fn set_t(&mut self, value: T) {
        self.top_left = Some(value);
        self.top_right = Some(value);
    }

    pub fn set_r(&mut self, value: T) {
        self.top_right = Some(value);
        self.bottom_right = Some(value);
    }

    pub fn set_b(&mut self, value: T) {
        self.bottom_right = Some(value);
        self.bottom_left = Some(value);
    }

    pub fn set_l(&mut self, value: T) {
        self.top_left = Some(value);
        self.bottom_left = Some(value);
    }

    pub fn set_tl(&mut self, value: T) {
        self.top_left = Some(value);
    }

    pub fn set_tr(&mut self, value: T) {
        self.top_right = Some(value);
    }

    pub fn set_br(&mut self, value: T) {
        self.bottom_right = Some(value);
    }

    pub fn set_bl(&mut self, value: T) {
        self.bottom_left = Some(value);
    }
}

impl<T: PartialEq + Display> PropertyDisplay for Corners<T> {
    fn fmt_property(&self, name: &str, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match (
            &self.top_left,
            &self.top_right,
            &self.bottom_right,
            &self.bottom_left,
        ) {
            (Some(top_left), Some(top_right), Some(bottom_right), Some(bottom_left))
                if top_left == top_right
                    && top_right == bottom_right
                    && bottom_right == bottom_left =>
            {
                writeln!(f, "{name}: {top_left};")
            }
            (Some(top_left), Some(top_right), Some(bottom_right), Some(bottom_left))
                if top_left == bottom_right && bottom_left == top_right =>
            {
                // top-left-and-bottom-right | top-right-and-bottom-left
                writeln!(f, "{name}: {top_left} {top_right};")
            }
            (Some(top_left), Some(top_right), Some(bottom_right), Some(bottom_left))
                if bottom_left == top_right =>
            {
                // top-left | top-right-and-bottom-left | bottom-right
                writeln!(f, "{name}: {top_left} {top_right} {bottom_right};")
            }
            (Some(top_left), Some(top_right), Some(bottom_right), Some(bottom_left)) => {
                writeln!(
                    f,
                    "{name}: {top_left} {top_right} {bottom_right} {bottom_left};"
                )
            }
            (top_left, top_right, bottom_right, bottom_left) => {
                let (name, suffix) = if let Some((prefix, _)) = name.split_once('-') {
                    (prefix, &name[prefix.len()..])
                } else {
                    (name, "")
                };
                if let Some(top_left) = top_left {
                    writeln!(f, "{name}-top-left{suffix}: {top_left};")?
                }
                if let Some(top_right) = top_right {
                    writeln!(f, "{name}-right-right{suffix}: {top_right};")?
                }
                if let Some(bottom_right) = bottom_right {
                    writeln!(f, "{name}-bottom-right{suffix}: {bottom_right};")?
                }
                if let Some(bottom_left) = bottom_left {
                    writeln!(f, "{name}-bottom-left{suffix}: {bottom_left};")?
                }
                Ok(())
            }
        }
    }
}

impl<T> Default for Corners<T> {
    fn default() -> Self {
        Self {
            top_left: None,
            top_right: None,
            bottom_right: None,
            bottom_left: None,
        }
    }
}
