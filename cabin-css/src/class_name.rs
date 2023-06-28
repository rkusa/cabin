use std::borrow::Cow;
use std::fmt;
use std::ops::{Add, AddAssign};

#[derive(Default)]
pub struct ClassName<'a>(pub Option<Cow<'a, str>>);

impl<'a> ClassName<'a> {
    pub fn append(self, other: ClassName<'a>) -> ClassName<'a> {
        self + other
    }

    pub fn append_when(self, condition: bool, other: ClassName<'a>) -> ClassName<'a> {
        if !condition {
            self
        } else {
            self + other
        }
    }
}

impl<'a> From<ClassName<'a>> for Cow<'a, str> {
    fn from(value: ClassName<'a>) -> Self {
        value.0.unwrap_or_default()
    }
}

impl<'a> From<Option<&'a str>> for ClassName<'a> {
    fn from(value: Option<&'a str>) -> Self {
        ClassName(value.map(Cow::Borrowed))
    }
}

impl<'a> Add<ClassName<'a>> for ClassName<'a> {
    type Output = ClassName<'a>;

    fn add(self, rhs: ClassName<'a>) -> Self::Output {
        // FIXME: avoid allocation
        ClassName(Some(Cow::Owned(format!("{self} {rhs}"))))
    }
}

impl<'a> Add<Option<ClassName<'a>>> for ClassName<'a> {
    type Output = ClassName<'a>;

    fn add(self, rhs: Option<ClassName<'a>>) -> Self::Output {
        if let Some(rhs) = rhs {
            // FIXME: avoid allocation
            ClassName(Some(Cow::Owned(format!("{self} {rhs}"))))
        } else {
            self
        }
    }
}

impl<'a> Add<ClassName<'a>> for Option<ClassName<'a>> {
    type Output = ClassName<'a>;

    fn add(self, rhs: ClassName<'a>) -> Self::Output {
        if let Some(lhs) = self {
            // FIXME: avoid allocation
            ClassName(Some(Cow::Owned(format!("{lhs} {rhs}"))))
        } else {
            rhs
        }
    }
}

impl<'a> AddAssign for ClassName<'a> {
    fn add_assign(&mut self, rhs: Self) {
        // FIXME: avoid allocation
        *self = ClassName(Some(Cow::Owned(format!("{self} {rhs}"))))
    }
}

impl<'a> fmt::Display for ClassName<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.as_deref().unwrap_or_default().fmt(f)
    }
}
