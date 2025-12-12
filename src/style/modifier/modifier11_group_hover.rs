use crate::style::modifier::StyleModifier;

pub struct GroupHover;

impl StyleModifier for GroupHover {
    fn order(&self) -> usize {
        11
    }

    fn selector_prefix(&self, out: &mut dyn std::fmt::Write) -> std::fmt::Result {
        out.write_str(".group:hover ")
    }
}
