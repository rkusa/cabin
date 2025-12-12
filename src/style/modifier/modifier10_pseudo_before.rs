use crate::style::modifier::StyleModifier;

pub struct PseudoBefore;

impl StyleModifier for PseudoBefore {
    fn order(&self) -> usize {
        10
    }

    fn selector_suffix(&self, out: &mut dyn std::fmt::Write) -> std::fmt::Result {
        out.write_str("::before")
    }
}
