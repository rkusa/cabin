use std::fmt;

use crate::Utility;

pub struct MaxContainerWidth<S> {
    pub max_width_px: u32,
    pub style: S,
}

impl<S> MaxContainerWidth<S> {
    pub fn new(max_width_px: u32, style: S) -> Self {
        Self {
            max_width_px,
            style,
        }
    }
}

impl<S: Utility> Utility for MaxContainerWidth<S> {
    fn declarations(&self, f: &mut dyn fmt::Write) -> fmt::Result {
        self.style.declarations(f)
    }

    fn selector_prefix(&self, f: &mut dyn fmt::Write) -> fmt::Result {
        write!(f, "@container (max-width: {}px) {{ ", self.max_width_px)?;
        self.style.selector_prefix(f)?;
        Ok(())
    }

    fn selector_suffix(&self, f: &mut dyn fmt::Write) -> fmt::Result {
        self.style.selector_suffix(f)
    }

    fn selector_declarations(&self, f: &mut dyn fmt::Write) -> fmt::Result {
        self.style.selector_declarations(f)
    }

    fn suffix(&self, f: &mut dyn fmt::Write) -> fmt::Result {
        f.write_str("} ")?;
        self.style.suffix(f)?;
        Ok(())
    }

    fn write_animate_from(&self, f: &mut dyn fmt::Write) -> fmt::Result {
        self.style.write_animate_from(f)
    }

    fn write_animate_to(&self, f: &mut dyn fmt::Write) -> fmt::Result {
        self.style.write_animate_to(f)
    }

    fn hash_modifier(&self, hasher: &mut dyn std::hash::Hasher) {
        hasher.write(b"max-width");
        hasher.write(&self.max_width_px.to_be_bytes());
        self.style.hash_modifier(hasher);
    }

    fn order(&self) -> usize {
        self.style
            .order()
            .saturating_add(9999usize.saturating_sub(self.max_width_px as usize))
    }
}
