use self::internal::{FontSize, TextColor};
use crate::Length;

pub const BLACK: TextColor = TextColor::custom("black");
pub const WHITE: TextColor = TextColor::custom("white");

pub const XS: FontSize = FontSize::custom(Length::Rem(0.75), Length::Rem(1.0));
pub const SM: FontSize = FontSize::custom(Length::Rem(0.875), Length::Rem(1.25));

pub fn color(color: &'static str) -> TextColor {
    TextColor::custom(color)
}

mod internal {
    use std::fmt;

    use crate::{Length, Style};

    pub struct TextColor(&'static str);

    impl TextColor {
        pub const fn custom(color: &'static str) -> Self {
            Self(color)
        }
    }

    impl Style for TextColor {
        fn declarations(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            writeln!(f, "color: {};", self.0)
        }
    }

    pub struct FontSize {
        font_size: Length,
        line_height: Length,
    }

    impl FontSize {
        pub const fn custom(font_size: Length, line_height: Length) -> Self {
            Self {
                font_size,
                line_height,
            }
        }
    }

    impl Style for FontSize {
        fn declarations(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            writeln!(f, "font-size: {};", self.font_size)?;
            writeln!(f, "line-height: {};", self.line_height)?;
            Ok(())
        }
    }
}
