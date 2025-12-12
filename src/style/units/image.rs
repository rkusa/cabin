use std::fmt;

use crate::style::property_display::PropertyDisplay;
use crate::style::units::gradient::Gradient;

#[derive(Default)]
pub enum Image {
    #[default]
    None,
    LinearGradient(Gradient),
}

impl Image {
    pub fn set_none(&mut self) {
        *self = Self::None;
    }

    pub fn linear_gradient(&mut self) -> &mut Gradient {
        if let Image::None = self {
            *self = Self::LinearGradient(Default::default());
        }
        let Image::LinearGradient(gradient) = self else {
            unreachable!();
        };
        gradient
    }
}

impl PropertyDisplay for Image {
    fn fmt_property(&self, name: &str, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Image::None => writeln!(f, "{name}: none;"),
            Image::LinearGradient(gradient) => gradient.fmt_property(name, f),
        }
    }
}
