use crate::style::modifier::StyleModifier;

pub struct PseudoDisabled;

impl StyleModifier for PseudoDisabled {
    fn order(&self) -> usize {
        2
    }

    fn selector_suffix(&self, out: &mut dyn std::fmt::Write) -> std::fmt::Result {
        out.write_str(":disabled")
    }
}
