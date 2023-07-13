use std::fmt;

use crate::Utility;

pub struct Before<S>(pub S);

impl<S: Utility> Utility for Before<S> {
    fn declarations(&self, f: &mut dyn fmt::Write) -> fmt::Result {
        // TODO: only once for multiple after/before styles
        writeln!(f, "content: var(--tw-content);")?;
        self.0.declarations(f)
    }

    fn selector_suffix(&self, f: &mut dyn fmt::Write) -> fmt::Result {
        f.write_str("::before")?;
        self.0.selector_suffix(f)
    }

    fn hash_modifier(&self, hasher: &mut dyn std::hash::Hasher) {
        hasher.write(b"before");
        self.0.hash_modifier(hasher);
    }

    fn order(&self) -> usize {
        self.0.order()
    }
}
