use crate::style::modifier::StyleModifier;

pub struct PseudoFocusVisible;

impl StyleModifier for PseudoFocusVisible {
    fn order(&self) -> usize {
        5
    }

    fn selector_suffix(&self, out: &mut dyn std::fmt::Write) -> std::fmt::Result {
        out.write_str(":focus-visible")
    }
}
