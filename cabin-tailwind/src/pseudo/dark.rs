use std::fmt;

use crate::Utility;

pub struct Dark<S> {
    pub style: S,
}

impl<S> Dark<S> {
    pub fn new(style: S) -> Self {
        Self { style }
    }
}

impl<S: Utility> Utility for Dark<S> {
    fn declarations(&self, f: &mut dyn fmt::Write) -> fmt::Result {
        self.style.declarations(f)
    }

    fn selector_prefix(&self, f: &mut dyn fmt::Write) -> fmt::Result {
        f.write_str("@media (prefers-color-scheme: dark) { ")?;
        self.style.selector_prefix(f)?;
        Ok(())
    }

    fn selector_declarations(&self, f: &mut dyn fmt::Write) -> fmt::Result {
        self.style.selector_declarations(f)
    }

    fn suffix(&self, f: &mut dyn fmt::Write) -> fmt::Result {
        f.write_str("}")?;
        self.style.suffix(f)?;
        Ok(())
    }

    fn hash_modifier(&self, hasher: &mut dyn std::hash::Hasher) {
        hasher.write(b"dark");
        self.style.hash_modifier(hasher);
    }

    fn order(&self) -> usize {
        self.style.order()
    }
}
