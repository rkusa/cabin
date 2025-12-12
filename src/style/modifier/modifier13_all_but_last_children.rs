use crate::style::modifier::StyleModifier;

pub struct AllButLastChildren;

impl StyleModifier for AllButLastChildren {
    fn order(&self) -> usize {
        13
    }

    fn selector_prefix(&self, out: &mut dyn std::fmt::Write) -> std::fmt::Result {
        out.write_str(":where(")
    }

    fn selector_suffix(&self, out: &mut dyn std::fmt::Write) -> std::fmt::Result {
        out.write_str(" > :not(:last-child))")
    }
}
