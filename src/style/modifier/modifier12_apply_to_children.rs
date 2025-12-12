use crate::style::modifier::StyleModifier;

pub struct ApplyToChildren;

impl StyleModifier for ApplyToChildren {
    fn order(&self) -> usize {
        12
    }

    fn selector_suffix(&self, out: &mut dyn std::fmt::Write) -> std::fmt::Result {
        out.write_str(" > *")
    }
}
