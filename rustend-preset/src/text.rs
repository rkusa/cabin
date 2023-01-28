use rustend_css::text::{FontSize, TextColor};
use rustend_css::Length;

pub const BLACK: TextColor = TextColor::custom("black");

pub const XS: FontSize = FontSize::custom(Length::Rem(0.75), Length::Rem(1.0));

pub const SM: FontSize = FontSize::custom(Length::Rem(0.875), Length::Rem(1.25));

pub fn color(color: &'static str) -> TextColor {
    TextColor::custom(color)
}
