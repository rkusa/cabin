use std::fmt;

use crate::Utility;

pub struct Custom<S>(pub &'static str, pub S);

impl<S: Utility> Utility for Custom<S> {
    fn declarations(&self, f: &mut dyn fmt::Write) -> fmt::Result {
        self.1.declarations(f)
    }

    fn selector_suffix(&self, f: &mut dyn fmt::Write) -> fmt::Result {
        write!(f, "::{}", self.0)?;
        self.1.selector_suffix(f)
    }

    fn selector_declarations(&self, f: &mut dyn fmt::Write) -> fmt::Result {
        self.1.selector_declarations(f)
    }

    fn hash_modifier(&self, hasher: &mut dyn std::hash::Hasher) {
        hasher.write(self.0.as_bytes());
        self.1.hash_modifier(hasher);
    }

    fn order(&self) -> usize {
        self.1.order()
    }
}
