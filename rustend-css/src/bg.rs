use self::internal::BgColor;

pub const BLACK: BgColor = BgColor("black");

pub fn color(color: &'static str) -> BgColor {
    BgColor(color)
}

mod internal {
    use std::fmt;

    use crate::Style;

    pub struct BgColor(pub(super) &'static str);

    impl Style for BgColor {
        fn declarations(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            writeln!(f, "background-color: {};", self.0)
        }
    }
}
