use rustend::style::length::Length;
use rustend::style::text::{FontSize, TextColor};

pub const BLACK: TextColor = TextColor::custom("black");

pub const XS: FontSize = FontSize::custom(Length::Rem(0.75), Length::Rem(1.0));

pub const SM: FontSize = FontSize::custom(Length::Rem(0.875), Length::Rem(1.25));