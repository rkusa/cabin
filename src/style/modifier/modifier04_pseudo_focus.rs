use crate::style::modifier::StyleModifier;

pub struct PseudoFocus;

impl StyleModifier for PseudoFocus {
    fn order(&self) -> usize {
        4
    }

    fn selector_suffix(&self, out: &mut dyn std::fmt::Write) -> std::fmt::Result {
        out.write_str(":focus")
    }
}
