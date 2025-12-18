use std::fmt;

use crate::style::property_display::PropertyDisplay;
use crate::style::style_definition::MergeFrom;
use crate::style::units::float::Float;
use crate::style::units::length::Length;

#[derive(Default, Clone, Hash, PartialEq, Eq)]
pub struct BoxShadow {
    pub ring: Option<Ring>,
    pub shadow_kind: Option<ShadowKind>,
    pub shadow_color: Option<&'static str>,
}

#[derive(Clone, Hash, PartialEq, Eq)]
pub struct Ring {
    pub inset: bool,
    pub width: Length,
    pub color: &'static str,
}

#[derive(Default, Clone, Copy, Hash, PartialEq, Eq)]
pub enum ShadowKind {
    Xs2,
    Xs,
    #[default]
    Sm,
    Md,
    Lg,
    Xl,
    Xl2,
}

impl PropertyDisplay for BoxShadow {
    fn fmt_property(&self, name: &str, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{name}: ")?;
        if let Some(ring) = &self.ring {
            write!(f, "{ring}")?;
        }

        if let Some(shadow_kind) = self.shadow_kind {
            if self.ring.is_some() {
                write!(f, ", ")?;
            }

            match shadow_kind {
                ShadowKind::Xs2 => write!(
                    f,
                    "0 1px {color}",
                    color = self.shadow_color.unwrap_or("rgb(0 0 0 / 0.05)")
                )?,
                ShadowKind::Xs => write!(
                    f,
                    "0 1px 2px 0 {color}",
                    color = self.shadow_color.unwrap_or("rgb(0 0 0 / 0.05)")
                )?,
                ShadowKind::Sm => write!(
                    f,
                    "0 1px 3px 0 {color}, 0 1px 2px -1px {color}",
                    color = self.shadow_color.unwrap_or("rgb(0 0 0 / 0.1)")
                )?,
                ShadowKind::Md => write!(
                    f,
                    "0 4px 6px -1px {color}, 0 2px 4px -2px {color}",
                    color = self.shadow_color.unwrap_or("rgb(0 0 0 / 0.1)")
                )?,
                ShadowKind::Lg => write!(
                    f,
                    "0 10px 15px -3px {color}, 0 4px 6px -4px {color}",
                    color = self.shadow_color.unwrap_or("rgb(0 0 0 / 0.1)")
                )?,
                ShadowKind::Xl => write!(
                    f,
                    "0 20px 25px -5px {color}, 0 8px 10px -6px {color}",
                    color = self.shadow_color.unwrap_or("rgb(0 0 0 / 0.1)")
                )?,
                ShadowKind::Xl2 => write!(
                    f,
                    "0 25px 50px -12px {color}",
                    color = self.shadow_color.unwrap_or("rgb(0 0 0 / 0.25)")
                )?,
            }
        }

        write!(f, ";")?;
        Ok(())
    }
}

impl fmt::Display for Ring {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.inset {
            write!(f, "inset ")?;
        }
        // Use calc to support different length units
        write!(f, "0 0 0 {} {}", self.width, self.color)
    }
}

impl Default for Ring {
    fn default() -> Self {
        Self {
            inset: false,
            width: Length::Px(Float::from(1i32)),
            color: "currentColor",
        }
    }
}

impl MergeFrom for BoxShadow {
    fn merge_from(&mut self, other: Self) {
        let Self {
            ring,
            shadow_kind,
            shadow_color,
        } = other;
        self.ring.merge_from(ring);
        self.shadow_kind.merge_from(shadow_kind);
        self.shadow_color.merge_from(shadow_color);
    }
}

impl MergeFrom for Ring {
    fn merge_from(&mut self, other: Self) {
        let Self {
            inset,
            width,
            color,
        } = other;
        self.inset.merge_from(inset);
        self.width.merge_from(width);
        self.color.merge_from(color);
    }
}

impl MergeFrom for ShadowKind {
    fn merge_from(&mut self, other: Self) {
        *self = other;
    }
}
