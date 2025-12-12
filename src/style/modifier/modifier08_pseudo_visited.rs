use crate::style::modifier::StyleModifier;

pub struct PseudoVisited;

impl StyleModifier for PseudoVisited {
    fn order(&self) -> usize {
        8
    }

    fn selector_suffix(&self, out: &mut dyn std::fmt::Write) -> std::fmt::Result {
        out.write_str(":visited")
    }
}
