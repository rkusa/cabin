use std::fmt;

use crate::style::property_display::PropertyDisplay;

#[derive(Clone, Copy, PartialEq)]
pub enum Either<L, R = &'static str> {
    Left(L),
    Right(R),
}

impl<L: fmt::Display, R: fmt::Display> PropertyDisplay for Either<L, R> {
    fn fmt_property(&self, name: &str, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Either::Left(length) => writeln!(f, "{name}: {length};"),
            Either::Right(custom) => writeln!(f, "{name}: {custom};"),
        }
    }
}

impl<L: fmt::Display, R: fmt::Display> fmt::Display for Either<L, R> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Either::Left(t) => t.fmt(f),
            Either::Right(c) => c.fmt(f),
        }
    }
}
