use crate::style::modifier::StyleModifier;

pub struct PseudoHover;

impl StyleModifier for PseudoHover {
    fn order(&self) -> usize {
        7
    }

    fn selector_suffix(&self, out: &mut dyn std::fmt::Write) -> std::fmt::Result {
        out.write_str(":hover")
    }
}
