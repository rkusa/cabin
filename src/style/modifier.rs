use std::cmp::{Ordering, Reverse};

#[derive(Default, Clone, Hash, PartialEq, Eq)]
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
    pub max_width: Option<u32>,
    pub min_width: Option<u32>,
    pub max_container_width: Option<u32>,
    pub min_container_width: Option<u32>,
    pub print: bool,
    pub dark: bool,
    pub other_pseudo_element: Option<&'static str>,
}

impl StyleModifier {
    pub fn merge_into(&self, other: &mut Self) {
        other.active = self.active || other.active;
        other.disabled = self.disabled || other.disabled;
        other.enabled = self.enabled || other.enabled;
        other.focus = self.focus || other.focus;
        other.focus_visible = self.focus_visible || other.focus_visible;
        other.focus_within = self.focus_within || other.focus_within;
        other.hover = self.hover || other.hover;
        other.visited = self.visited || other.visited;
        other.after = self.after || other.after;
        other.before = self.before || other.before;
        other.group_hover = self.group_hover || other.group_hover;
        other.all_children = self.all_children || other.all_children;
        other.all_but_last_children = self.all_but_last_children || other.all_but_last_children;
        other.max_width = self.max_width.or(other.max_width);
        other.min_width = self.min_width.or(other.min_width);
        other.max_container_width = self.max_container_width.or(other.max_container_width);
        other.min_container_width = self.min_container_width.or(other.min_container_width);
        other.print = self.print || other.print;
        other.dark = self.dark || other.dark;
        other.other_pseudo_element = self.other_pseudo_element.or(other.other_pseudo_element);
    }
}

impl StyleModifier {
    /// The variants that don't introduce an at-rule as bits, in the order Tailwind (v4) sorts
    /// them, so that e.g. `:disabled` rules come after (and thus override) `:hover` rules.
    /// Compared as a number, the latest variant decides.
    fn variant_rank(&self) -> u16 {
        let variants = [
            self.all_children,
            self.all_but_last_children,
            self.group_hover,
            self.other_pseudo_element.is_some(),
            self.before,
            self.after,
            self.visited,
            self.focus_within,
            self.hover,
            self.focus,
            self.focus_visible,
            self.active,
            self.enabled,
            self.disabled,
        ];
        variants
            .into_iter()
            .enumerate()
            .fold(0, |rank, (i, set)| rank | (u16::from(set) << i))
    }
}

impl Ord for StyleModifier {
    /// Rules are written in this order, so later (greater) modifiers override earlier ones. Set
    /// modifiers come after unset ones. The order is total (consistent with `Eq`), so that the
    /// CSS output is deterministic.
    fn cmp(&self, other: &Self) -> Ordering {
        self.print
            .cmp(&other.print)
            .then(self.dark.cmp(&other.dark))
            .then(self.min_width.cmp(&other.min_width))
            // Narrower max widths come later, so they override wider ones.
            .then(
                self.max_width
                    .map(Reverse)
                    .cmp(&other.max_width.map(Reverse)),
            )
            .then(self.min_container_width.cmp(&other.min_container_width))
            .then(
                self.max_container_width
                    .map(Reverse)
                    .cmp(&other.max_container_width.map(Reverse)),
            )
            .then(self.variant_rank().cmp(&other.variant_rank()))
            .then(self.other_pseudo_element.cmp(&other.other_pseudo_element))
    }
}

impl PartialOrd<StyleModifier> for StyleModifier {
    fn partial_cmp(&self, other: &StyleModifier) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
