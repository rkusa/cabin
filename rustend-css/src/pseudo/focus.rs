use std::fmt;

use crate::Style;

pub struct Focus<S>(pub S);

impl<S: Style> Style for Focus<S> {
    fn declarations(&self, f: &mut dyn fmt::Write) -> fmt::Result {
        self.0.declarations(f)
    }

    fn selector_prefix(&self, f: &mut dyn fmt::Write) -> fmt::Result {
        f.write_str(":focus")?;
        self.0.selector_prefix(f)
    }

    fn hash_modifier(&self, hasher: &mut dyn std::hash::Hasher) {
        hasher.write(b"focus");
        self.0.hash_modifier(hasher);
    }
}
