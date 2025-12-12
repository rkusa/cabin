use crate::style::modifier::StyleModifier;

pub struct PseudoAfter;

impl StyleModifier for PseudoAfter {
    fn order(&self) -> usize {
        9
    }

    fn selector_suffix(&self, out: &mut dyn std::fmt::Write) -> std::fmt::Result {
        out.write_str("::after")
    }
}
