use std::fmt;

use crate::Utility;

pub struct Print<S> {
    pub style: S,
}

impl<S> Print<S> {
    pub fn new(style: S) -> Self {
        Self { style }
    }
}

impl<S: Utility> Utility for Print<S> {
    fn declarations(&self, f: &mut dyn fmt::Write) -> fmt::Result {
        self.style.declarations(f)
    }

    fn selector_prefix(&self, f: &mut dyn fmt::Write) -> fmt::Result {
        f.write_str("@media print { ")?;
        self.style.selector_prefix(f)?;
        Ok(())
    }

    fn suffix(&self, f: &mut dyn fmt::Write) -> fmt::Result {
        f.write_str("} ")?;
        self.style.suffix(f)?;
        Ok(())
    }

    fn hash_modifier(&self, hasher: &mut dyn std::hash::Hasher) {
        hasher.write(b"print");
        self.style.hash_modifier(hasher);
    }

    fn order(&self) -> usize {
        // Move to the end of the stylesheet to take precedence
        self.style.order().max(5000)
    }
}