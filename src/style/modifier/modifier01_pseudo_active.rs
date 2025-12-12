use crate::style::modifier::StyleModifier;

pub struct PseudoActive;

impl StyleModifier for PseudoActive {
    fn order(&self) -> usize {
        1
    }

    fn selector_suffix(&self, out: &mut dyn std::fmt::Write) -> std::fmt::Result {
        out.write_str(":active")
    }
}
