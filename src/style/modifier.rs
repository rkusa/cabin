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
    pub max_width: Option<u32>,
    pub min_width: Option<u32>,
    pub max_container_width: Option<u32>,
    pub min_container_width: Option<u32>,
    pub print: bool,
    pub dark: bool,
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
            max_width: self.max_width.or(other.max_width),
            min_width: self.min_width.or(other.min_width),
            max_container_width: self.max_container_width.or(other.max_container_width),
            min_container_width: self.min_container_width.or(other.min_container_width),
            print: self.print || other.print,
            dark: self.dark || other.dark,
        };
    }
}

impl Ord for StyleModifier {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.print && !other.print {
            Ordering::Greater
        } else if !self.print && other.print {
            Ordering::Less
        } else if self.dark && !other.dark {
            Ordering::Greater
        } else if !self.dark && other.dark {
            Ordering::Less
        } else if let Some((a, b)) = self.min_width.zip(other.min_width) {
            a.cmp(&b)
        } else if self.min_width.is_some() && other.min_width.is_none() {
            Ordering::Less
        } else if self.min_width.is_none() && other.min_width.is_some() {
            Ordering::Greater
        } else if let Some((a, b)) = self.max_width.zip(other.max_width) {
            a.cmp(&b)
        } else if self.max_width.is_some() && other.max_width.is_none() {
            Ordering::Greater
        } else if self.max_width.is_none() && other.max_width.is_some() {
            Ordering::Less
        } else if let Some((a, b)) = self.min_container_width.zip(other.min_container_width) {
            a.cmp(&b)
        } else if self.min_container_width.is_some() && other.min_container_width.is_none() {
            Ordering::Less
        } else if self.min_container_width.is_none() && other.min_container_width.is_some() {
            Ordering::Greater
        } else if let Some((a, b)) = self.max_container_width.zip(other.max_container_width) {
            a.cmp(&b)
        } else if self.max_container_width.is_some() && other.max_container_width.is_none() {
            Ordering::Greater
        } else if self.max_container_width.is_none() && other.max_container_width.is_some() {
            Ordering::Less
        } else {
            Ordering::Equal
        }
    }
}

impl PartialOrd<StyleModifier> for StyleModifier {
    fn partial_cmp(&self, other: &StyleModifier) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
