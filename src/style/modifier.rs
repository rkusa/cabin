use std::cmp::Ordering;

#[derive(Default, Clone, PartialEq, Eq)]
pub struct StyleModifier {
    pub active: bool,
    pub disabled: bool,
    pub enabled: bool,
    pub focus: bool,
    pub focus_visible: bool,
    pub focus_within: bool,
    pub hover: bool,
    pub visited: bool,
    pub after: bool,
    pub before: bool,
    pub group_hover: bool,
    pub all_children: bool,
    pub all_but_last_children: bool,
}

impl StyleModifier {
    pub fn merge_into(&self, other: &mut Self) {
        *other = Self {
            active: self.active || other.active,
            disabled: self.disabled || other.disabled,
            enabled: self.enabled || other.enabled,
            focus: self.focus || other.focus,
            focus_visible: self.focus_visible || other.focus_visible,
            focus_within: self.focus_within || other.focus_within,
            hover: self.hover || other.hover,
            visited: self.visited || other.visited,
            after: self.after || other.after,
            before: self.before || other.before,
            group_hover: self.group_hover || other.group_hover,
            all_children: self.all_children || other.all_children,
            all_but_last_children: self.all_but_last_children || other.all_but_last_children,
        };
    }
}

impl Ord for StyleModifier {
    fn cmp(&self, other: &Self) -> Ordering {
        // FIXME: order based on media / container queries
        Ordering::Equal
    }
}

impl PartialOrd<StyleModifier> for StyleModifier {
    fn partial_cmp(&self, other: &StyleModifier) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
