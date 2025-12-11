use std::fmt;

use crate::tailwind::Utility;

pub struct Custom<S>(pub &'static str, pub S);

impl<S: Utility> Utility for Custom<S> {
    fn declarations(&self, f: &mut dyn fmt::Write) -> fmt::Result {
        self.1.declarations(f)
    }

    fn selector_prefix(&self, f: &mut dyn fmt::Write) -> fmt::Result {
        self.1.selector_prefix(f)
    }

    fn selector_suffix(&self, f: &mut dyn fmt::Write) -> fmt::Result {
        write!(f, "::{}", self.0)?;
        self.1.selector_suffix(f)
    }

    fn selector_declarations(&self, f: &mut dyn fmt::Write) -> fmt::Result {
        self.1.selector_declarations(f)
    }

    fn suffix(&self, f: &mut dyn fmt::Write) -> fmt::Result {
        self.1.suffix(f)
    }

    fn write_animate_from(&self, f: &mut dyn fmt::Write) -> fmt::Result {
        self.1.write_animate_from(f)
    }

    fn write_animate_to(&self, f: &mut dyn fmt::Write) -> fmt::Result {
        self.1.write_animate_to(f)
    }

    fn hash_modifier(&self, hasher: &mut dyn std::hash::Hasher) {
        hasher.write(self.0.as_bytes());
        self.1.hash_modifier(hasher);
    }

    fn order(&self) -> usize {
        self.1.order()
    }
}
