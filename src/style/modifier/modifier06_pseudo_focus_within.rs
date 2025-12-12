use crate::style::modifier::StyleModifier;

pub struct PseudoFocusWithin;

impl StyleModifier for PseudoFocusWithin {
    fn order(&self) -> usize {
        6
    }

    fn selector_suffix(&self, out: &mut dyn std::fmt::Write) -> std::fmt::Result {
        out.write_str(":focus-within")
    }
}
