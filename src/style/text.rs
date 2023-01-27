use std::fmt;

use super::Style;

pub trait TextStyle {
    fn css(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result;
}

pub fn text(style: impl TextStyle) -> impl Style {
    internal::TextStyle(style)
}

mod internal {
    use std::fmt;

    use crate::style::Style;

    pub struct TextStyle<S: super::TextStyle>(pub S);

    impl<S: super::TextStyle> Style for TextStyle<S> {
        fn css(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            self.0.css(f)
        }
    }
}
