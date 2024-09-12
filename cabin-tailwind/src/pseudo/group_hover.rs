use std::fmt;

use crate::Utility;

pub struct GroupHover<S>(pub S);

impl<S: Utility> Utility for GroupHover<S> {
    fn declarations(&self, f: &mut dyn fmt::Write) -> fmt::Result {
        self.0.declarations(f)
    }

    fn selector_prefix(&self, f: &mut dyn fmt::Write) -> fmt::Result {
        f.write_str(".group:hover ")?;
        Ok(())
    }

    fn selector_declarations(&self, f: &mut dyn fmt::Write) -> fmt::Result {
        self.0.selector_declarations(f)
    }

    fn hash_modifier(&self, hasher: &mut dyn std::hash::Hasher) {
        hasher.write(b".group:hover");
        self.0.hash_modifier(hasher);
    }

    fn order(&self) -> usize {
        self.0.order()
    }
}
