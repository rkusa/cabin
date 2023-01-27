use super::color::Color;
use super::font::FontSize;
use super::length::Length;

pub const BLACK: Color = Color("black");

pub const XS: FontSize = FontSize {
    font_size: Length::Rem(0.75),
    line_height: Length::Rem(1.0),
};

pub const SM: FontSize = FontSize {
    font_size: Length::Rem(0.875),
    line_height: Length::Rem(1.25),
};

// impl TextStyle for Xs {
//     fn css(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         f.write_str("font-size: 0.75rem;")?;
//         f.write_str("line-height: 1rem;")?;
//         Ok(())
//     }
// }

// impl TextStyle for Sm {
//     fn css(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         f.write_str("font-size: 0.875rem;")?;
//         f.write_str("line-height: 1.25rem;")?;
//         Ok(())
//     }
// }
