use self::internal::BgColor;

pub const BLACK: BgColor = BgColor::custom("black");

pub fn color(color: &'static str) -> BgColor {
    BgColor::custom(color)
}

mod internal {
    use std::fmt;

    use crate::Style;

    pub struct BgColor(&'static str);

    impl BgColor {
        pub const fn custom(color: &'static str) -> Self {
            Self(color)
        }
    }

    impl Style for BgColor {
        fn declarations(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            writeln!(f, "background-color: {};", self.0)
        }
    }
}
