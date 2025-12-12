use crate::style::modifier::StyleModifier;

pub struct PseudoEnabled;

impl StyleModifier for PseudoEnabled {
    fn order(&self) -> usize {
        3
    }

    fn selector_suffix(&self, out: &mut dyn std::fmt::Write) -> std::fmt::Result {
        out.write_str(":enabled")
    }
}
