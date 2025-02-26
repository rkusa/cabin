mod pseudo;
pub mod registry;
pub mod utilities;

use std::fmt;
use std::hash::Hasher;

pub use cabin::html::elements::common::Class;
pub use cabin_macros::{STYLES, tw, tw0, tw2, tw3};
pub use utilities as css;

pub mod prelude {
    pub use crate::{Responsive, Utility, tw, tw0, tw2, tw3, utilities as tw};
}

pub trait Utility {
    fn declarations(&self, f: &mut dyn fmt::Write) -> fmt::Result;

    fn selector_prefix(&self, _f: &mut dyn fmt::Write) -> fmt::Result {
        Ok(())
    }

    fn selector_suffix(&self, _f: &mut dyn fmt::Write) -> fmt::Result {
        Ok(())
    }

    fn selector_declarations(&self, _f: &mut dyn fmt::Write) -> fmt::Result {
        Ok(())
    }

    fn suffix(&self, _f: &mut dyn fmt::Write) -> fmt::Result {
        Ok(())
    }

    fn write_animate_from(&self, _f: &mut dyn fmt::Write) -> fmt::Result {
        Ok(())
    }

    fn write_animate_to(&self, _f: &mut dyn fmt::Write) -> fmt::Result {
        Ok(())
    }

    fn hash_modifier(&self, _hasher: &mut dyn Hasher) {}

    fn override_class_name(&self) -> Option<&str> {
        None
    }

    /// The higher the returned number the later the style is positioned in the stylesheet to take
    /// precedence.
    fn order(&self) -> usize {
        0
    }

    /// Apply style only when the element is being pressed (`:active`).
    fn active(self) -> pseudo::active::Active<Self>
    where
        Self: Sized,
    {
        pseudo::active::Active(self)
    }

    /// Apply style to `::after` pseudo element.
    fn after(self) -> pseudo::after::After<Self>
    where
        Self: Sized,
    {
        pseudo::after::After(self)
    }

    fn animate_from(self) -> pseudo::animate_from::AnimateFrom<Self>
    where
        Self: Sized,
    {
        pseudo::animate_from::AnimateFrom(self)
    }

    fn animate_to(self) -> pseudo::animate_to::AnimateTo<Self>
    where
        Self: Sized,
    {
        pseudo::animate_to::AnimateTo(self)
    }

    /// Apply style to all direct children (`> *`).
    fn apply_to_children(self) -> pseudo::apply_to_children::ApplyToChildren<Self>
    where
        Self: Sized,
    {
        pseudo::apply_to_children::ApplyToChildren(self)
    }

    /// Apply style to `::before` pseudo element.
    fn before(self) -> pseudo::before::Before<Self>
    where
        Self: Sized,
    {
        pseudo::before::Before(self)
    }

    /// Apply style to custom `::{pseudo}` pseudo element.
    fn pseudo(self, pseudo: &'static str) -> pseudo::custom::Custom<Self>
    where
        Self: Sized,
    {
        pseudo::custom::Custom(pseudo, self)
    }

    /// Apply style only when the user agent requested a dark color theme.
    /// `@media (prefers-color-scheme: dark)`
    fn dark(self) -> pseudo::dark::Dark<Self>
    where
        Self: Sized,
    {
        pseudo::dark::Dark::new(self)
    }

    /// Apply style only when the element is disabled (`:disabled`).
    fn disabled(self) -> pseudo::disabled::Disabled<Self>
    where
        Self: Sized,
    {
        pseudo::disabled::Disabled(self)
    }

    /// Apply style only when the element is not disabled (`:enabled`).
    fn enabled(self) -> pseudo::enabled::Enabled<Self>
    where
        Self: Sized,
    {
        pseudo::enabled::Enabled(self)
    }

    /// Apply style only when the element has focus (`:foucs`).
    fn focus(self) -> pseudo::focus::Focus<Self>
    where
        Self: Sized,
    {
        pseudo::focus::Focus(self)
    }

    /// Apply style only when the element has been focused using the keyboard (`:foucs-visible`).
    fn focus_visible(self) -> pseudo::focus_visible::FocusVisible<Self>
    where
        Self: Sized,
    {
        pseudo::focus_visible::FocusVisible(self)
    }

    /// Apply style only when the element or one of its descendants has focus (`:foucs-within`).
    fn focus_within(self) -> pseudo::focus_within::FocusWithin<Self>
    where
        Self: Sized,
    {
        pseudo::focus_within::FocusWithin(self)
    }

    fn group_hover(self) -> pseudo::group_hover::GroupHover<Self>
    where
        Self: Sized,
    {
        pseudo::group_hover::GroupHover(self)
    }

    /// Apply style only when the user hovers over the element (`:hover`).
    fn hover(self) -> pseudo::hover::Hover<Self>
    where
        Self: Sized,
    {
        pseudo::hover::Hover(self)
    }

    /// Apply style only when the link has already been visited (`:visited`).
    fn visited(self) -> pseudo::visited::Visited<Self>
    where
        Self: Sized,
    {
        pseudo::visited::Visited(self)
    }

    /// Apply style only when browser width is at least `min_width_px`.
    /// `@media (min-width: {min_width_px}px)`
    fn min_width_px(self, min_width_px: u32) -> pseudo::min_width::MinWidth<Self>
    where
        Self: Sized,
    {
        pseudo::min_width::MinWidth::new(min_width_px, self)
    }

    /// Apply style only when browser width does not exceed `max_width_px`.
    /// `@media (max-width: {max_width_px}px)`
    fn max_width_px(self, max_width_px: u32) -> pseudo::max_width::MaxWidth<Self>
    where
        Self: Sized,
    {
        pseudo::max_width::MaxWidth::new(max_width_px, self)
    }

    /// Apply style only when container width is at least `min_width_px`.
    /// `@container (min-width: {min_width_px}px)`
    fn min_container_width_px(
        self,
        min_width_px: u32,
    ) -> pseudo::min_container_width::MinContainerWidth<Self>
    where
        Self: Sized,
    {
        pseudo::min_container_width::MinContainerWidth::new(min_width_px, self)
    }

    /// Apply style only when container width does not exceed `max_width_px`.
    /// `@container (max-width: {max_width_px}px)`
    fn max_container_width_px(
        self,
        max_width_px: u32,
    ) -> pseudo::max_container_width::MaxContainerWidth<Self>
    where
        Self: Sized,
    {
        pseudo::max_container_width::MaxContainerWidth::new(max_width_px, self)
    }

    /// Apply style only when printing.
    fn print(self) -> pseudo::print::Print<Self>
    where
        Self: Sized,
    {
        pseudo::print::Print::new(self)
    }
}

include!(concat!(env!("OUT_DIR"), "/responsive.rs"));

pub struct Property<V = &'static str, const ORDER: usize = 0>(
    pub(crate) &'static str,
    pub(crate) V,
);
pub struct PropertyTwice<V = &'static str, const ORDER: usize = 0>(
    pub(crate) &'static str,
    pub(crate) &'static str,
    pub(crate) V,
);

pub struct StaticClass(pub(crate) &'static str);

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Length {
    Auto,
    MinContent,
    MaxContent,
    FitContent,
    Vw(u16),
    Svw(u16),
    Lvw(u16),
    Dvw(u16),
    Vh(u16),
    Svh(u16),
    Lvh(u16),
    Dvh(u16),
    Px(f32),
    Em(f32),
    Rem(f32),
    Percent(f32),
    Mm(f32),
    Cm(f32),
}

impl Length {
    fn is_zero(&self) -> bool {
        match self {
            Length::Auto | Length::MinContent | Length::MaxContent | Length::FitContent => false,
            Length::Vw(v)
            | Length::Svw(v)
            | Length::Lvw(v)
            | Length::Dvw(v)
            | Length::Vh(v)
            | Length::Svh(v)
            | Length::Lvh(v)
            | Length::Dvh(v) => *v == 0,
            Length::Px(v)
            | Length::Em(v)
            | Length::Rem(v)
            | Length::Percent(v)
            | Length::Mm(v)
            | Length::Cm(v) => v.abs() < f32::EPSILON,
        }
    }
}

impl fmt::Display for Length {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.is_zero() {
            return f.write_str("0");
        }

        match self {
            Length::Auto => f.write_str("auto"),
            Length::MinContent => f.write_str("min-content"),
            Length::MaxContent => f.write_str("max-content"),
            Length::FitContent => f.write_str("fit-content"),
            Length::Vw(v) => write!(f, "{v}vw"),
            Length::Svw(v) => write!(f, "{v}svw"),
            Length::Lvw(v) => write!(f, "{v}lvw"),
            Length::Dvw(v) => write!(f, "{v}dvw"),
            Length::Vh(v) => write!(f, "{v}vh"),
            Length::Svh(v) => write!(f, "{v}svh"),
            Length::Lvh(v) => write!(f, "{v}lvh"),
            Length::Dvh(v) => write!(f, "{v}dvh"),
            Length::Px(v) => write!(f, "{v}px"),
            Length::Em(v) => write!(f, "{v}em"),
            Length::Rem(v) => write!(f, "{v}rem"),
            Length::Percent(v) => write!(f, "{v}%"),
            Length::Mm(v) => write!(f, "{v}mm"),
            Length::Cm(v) => write!(f, "{v}cm"),
        }
    }
}

impl<V: fmt::Display, const ORDER: usize> Utility for Property<V, ORDER> {
    fn declarations(&self, f: &mut dyn fmt::Write) -> fmt::Result {
        writeln!(f, "{}: {};", self.0, self.1)
    }

    fn order(&self) -> usize {
        ORDER
    }
}

impl<V: fmt::Display, const ORDER: usize> Utility for PropertyTwice<V, ORDER> {
    fn declarations(&self, f: &mut dyn fmt::Write) -> fmt::Result {
        writeln!(f, "{}: {};", self.0, self.2)?;
        writeln!(f, "{}: {};", self.1, self.2)?;
        Ok(())
    }

    fn order(&self) -> usize {
        ORDER
    }
}

impl Utility for StaticClass {
    fn declarations(&self, _: &mut dyn fmt::Write) -> fmt::Result {
        Ok(())
    }

    fn hash_modifier(&self, hasher: &mut dyn Hasher) {
        hasher.write(self.0.as_bytes());
    }

    fn override_class_name(&self) -> Option<&str> {
        Some("group")
    }
}
