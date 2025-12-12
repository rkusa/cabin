pub mod modifier01_pseudo_active;
pub mod modifier02_pseudo_disabled;
pub mod modifier03_pseudo_enabled;
pub mod modifier04_pseudo_focus;
pub mod modifier05_pseudo_focus_visible;
pub mod modifier06_pseudo_focus_within;
pub mod modifier07_pseudo_hover;
pub mod modifier08_pseudo_visited;
pub mod modifier09_pseudo_after;
pub mod modifier10_pseudo_before;
pub mod modifier11_group_hover;
pub mod modifier12_apply_to_children;
pub mod modifier13_all_but_last_children;

use std::fmt;
use std::hash::{DefaultHasher, Hasher as _};

use smallvec::SmallVec;

pub trait StyleModifier {
    fn order(&self) -> usize;

    fn selector_prefix(&self, _out: &mut dyn fmt::Write) -> fmt::Result {
        Ok(())
    }

    fn selector_suffix(&self, _out: &mut dyn fmt::Write) -> fmt::Result {
        Ok(())
    }
}

pub const PSEUDO_ACTIVE: &'static dyn StyleModifier = &modifier01_pseudo_active::PseudoActive;
pub const PSEUDO_DISABLED: &'static dyn StyleModifier = &modifier02_pseudo_disabled::PseudoDisabled;
pub const PSEUDO_ENABLED: &'static dyn StyleModifier = &modifier03_pseudo_enabled::PseudoEnabled;
pub const PSEUDO_FOCUS: &'static dyn StyleModifier = &modifier04_pseudo_focus::PseudoFocus;
pub const PSEUDO_FOCUS_VISIBLE: &'static dyn StyleModifier =
    &modifier05_pseudo_focus_visible::PseudoFocusVisible;
pub const PSEUDO_FOCUS_WITHIN: &'static dyn StyleModifier =
    &modifier06_pseudo_focus_within::PseudoFocusWithin;
pub const PSEUDO_HOVER: &'static dyn StyleModifier = &modifier07_pseudo_hover::PseudoHover;
pub const PSEUDO_VISITED: &'static dyn StyleModifier = &modifier08_pseudo_visited::PseudoVisited;
pub const PSEUDO_AFTER: &'static dyn StyleModifier = &modifier09_pseudo_after::PseudoAfter;
pub const PSEUDO_BEFORE: &'static dyn StyleModifier = &modifier10_pseudo_before::PseudoBefore;
pub const GROUP_HOVER: &'static dyn StyleModifier = &modifier11_group_hover::GroupHover;
pub const APPLY_TO_CHILDREN: &'static dyn StyleModifier =
    &modifier12_apply_to_children::ApplyToChildren;
pub const ALL_BUT_LAST_CHILDREN: &'static dyn StyleModifier =
    &modifier13_all_but_last_children::AllButLastChildren;

#[derive(Default)]
pub struct StyleModifiers {
    modifier: SmallVec<&'static dyn StyleModifier, 2>,
    hash: u64,
}

impl StyleModifiers {
    pub fn new(modifier: &[&'static dyn StyleModifier]) -> Self {
        let mut modifier = SmallVec::from(modifier);
        modifier.sort_by_key(|m| m.order());

        let mut modifiers = Self { modifier, hash: 0 };
        modifiers.update_hash();
        modifiers
    }

    fn update_hash(&mut self) {
        let mut writer = HashWriter(DefaultHasher::default());
        for modifier in &self.modifier {
            modifier.selector_prefix(&mut writer).unwrap();
            modifier.selector_suffix(&mut writer).unwrap();
        }
        self.hash = writer.0.finish();
    }

    pub fn iter(&self) -> impl Iterator<Item = &'static dyn StyleModifier> {
        self.modifier.iter().copied()
    }

    pub fn merge_into(&self, other: &mut Self) {
        other.modifier.extend_from_slice(&self.modifier);
        other.modifier.sort_by_key(|m| m.order());
        other.update_hash();
    }
}

impl PartialEq<StyleModifiers> for StyleModifiers {
    fn eq(&self, other: &StyleModifiers) -> bool {
        self.hash == other.hash
    }
}

struct HashWriter(DefaultHasher);

impl fmt::Write for HashWriter {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.0.write(s.as_bytes());
        Ok(())
    }
}
