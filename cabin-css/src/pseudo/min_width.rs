use std::fmt;

use crate::Style;

pub struct MinWidth<S> {
    pub min_width_px: u32,
    pub style: S,
}

impl<S> MinWidth<S> {
    pub fn new(min_width_px: u32, style: S) -> Self {
        Self {
            min_width_px,
            style,
        }
    }
}

impl<S: Style> Style for MinWidth<S> {
    fn declarations(&self, f: &mut dyn fmt::Write) -> fmt::Result {
        self.style.declarations(f)
    }

    fn selector_prefix(&self, f: &mut dyn fmt::Write) -> fmt::Result {
        write!(f, "@media (min-width: {}px) {{ ", self.min_width_px)
    }

    fn suffix(&self, f: &mut dyn fmt::Write) -> fmt::Result {
        f.write_str("} ")
    }

    fn hash_modifier(&self, hasher: &mut dyn std::hash::Hasher) {
        hasher.write(b"min-width");
        hasher.write(&self.min_width_px.to_be_bytes());
        self.style.hash_modifier(hasher);
    }
}
