use std::fmt;

use crate::Style;

pub struct Visited<S>(pub S);

impl<S: Style> Style for Visited<S> {
    fn declarations(&self, f: &mut dyn fmt::Write) -> fmt::Result {
        self.0.declarations(f)
    }

    fn selector_suffix(&self, f: &mut dyn fmt::Write) -> fmt::Result {
        f.write_str(":visited")?;
        self.0.selector_suffix(f)
    }

    fn hash_modifier(&self, hasher: &mut dyn std::hash::Hasher) {
        hasher.write(b"hover");
        self.0.hash_modifier(hasher);
    }
}
