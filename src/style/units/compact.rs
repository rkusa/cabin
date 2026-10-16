use std::fmt::{self, Display};

use crate::style::property_display::PropertyDisplay;
use crate::style::style_definition::MergeFrom;
use crate::style::units::corners::Corners;
use crate::style::units::four_sided::FourSided;
use crate::style::units::inlined::Inlined;

/// A value with four sides (or corners), that are usually all set to the same value. Stores that
/// common case inline and only boxes the sides when they differ, to keep properties small.
#[derive(Default, Clone, Hash, PartialEq, Eq)]
pub enum Compact<S: Sides> {
    #[default]
    Unset,
    All(S::Value),
    Mixed(Box<S>),
}

pub trait Sides: Default + PartialEq {
    type Value: Copy + PartialEq + std::hash::Hash + Eq;

    fn uniform(value: Self::Value) -> Self;

    /// The value of all sides, if they are all set to the same value.
    fn as_uniform(&self) -> Option<Self::Value>;
}

impl<S: Sides> Compact<S> {
    /// Set all sides to the same value.
    pub fn set(&mut self, value: S::Value) {
        *self = Self::All(value);
    }

    fn from_sides(sides: S) -> Self {
        if let Some(value) = sides.as_uniform() {
            Self::All(value)
        } else if sides == S::default() {
            Self::Unset
        } else {
            Self::Mixed(Box::new(sides))
        }
    }

    fn to_sides(&self) -> S
    where
        S: Clone,
    {
        match self {
            Self::Unset => S::default(),
            Self::All(value) => S::uniform(*value),
            Self::Mixed(sides) => (**sides).clone(),
        }
    }

    // Keeps the representation canonical (`All` whenever all sides are equal), as class names are
    // derived from the hash of a style.
    fn update(&mut self, f: impl FnOnce(&mut S)) {
        match self {
            Self::Mixed(sides) => {
                f(sides);
                if let Some(value) = sides.as_uniform() {
                    *self = Self::All(value);
                } else if **sides == S::default() {
                    *self = Self::Unset;
                }
            }
            Self::All(value) => {
                let mut sides = S::uniform(*value);
                f(&mut sides);
                *self = Self::from_sides(sides);
            }
            Self::Unset => {
                let mut sides = S::default();
                f(&mut sides);
                *self = Self::from_sides(sides);
            }
        }
    }
}

impl<S: Sides + PropertyDisplay> PropertyDisplay for Compact<S> {
    fn fmt_property(&self, name: &str, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Unset => S::default().fmt_property(name, f),
            Self::All(value) => S::uniform(*value).fmt_property(name, f),
            Self::Mixed(sides) => sides.fmt_property(name, f),
        }
    }
}

impl<S: Sides + Clone + MergeFrom> MergeFrom for Compact<S> {
    fn merge_from(&mut self, other: Self) {
        match other {
            Self::Unset => {}
            other => {
                let mut sides = self.to_sides();
                sides.merge_from(other.to_sides());
                *self = Self::from_sides(sides);
            }
        }
    }
}

macro_rules! sides {
    ($ty:ident { $a:ident, $b:ident, $c:ident, $d:ident } $($setter:ident)*) => {
        impl<T: Copy + PartialEq + std::hash::Hash + Eq> Sides for $ty<T> {
            type Value = T;

            fn uniform(value: T) -> Self {
                let mut sides = Self::default();
                sides.set(value);
                sides
            }

            fn as_uniform(&self) -> Option<T> {
                match (self.$a, self.$b, self.$c, self.$d) {
                    (Some(a), Some(b), Some(c), Some(d)) if a == b && b == c && c == d => Some(a),
                    _ => None,
                }
            }
        }

        impl<T: Copy + PartialEq + std::hash::Hash + Eq + Display> Compact<$ty<T>> {
            $(
                pub fn $setter(&mut self, value: T) {
                    self.update(|sides| sides.$setter(value));
                }
            )*
        }
    };
}

sides!(Corners { top_left, top_right, bottom_right, bottom_left }
    set_t set_r set_b set_l set_tl set_tr set_br set_bl);
sides!(FourSided { top, right, bottom, left }
    set_x set_y set_top set_right set_bottom set_left);
sides!(Inlined { block_start, inline_end, block_end, inline_start }
    set_x set_y set_block_start set_inline_end set_block_end set_inline_start);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::style::units::length::Length;

    struct Css<'a, T>(&'a T);

    impl<T: PropertyDisplay> Display for Css<'_, T> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            self.0.fmt_property("border-radius", f)
        }
    }

    fn px(px: i16) -> Length {
        Length::Px(px.into())
    }

    #[test]
    fn collapses_to_all() {
        let mut c = Compact::<Corners<Length>>::default();
        c.set_t(px(2));
        assert!(matches!(c, Compact::Mixed(_)));
        c.set_b(px(2));
        assert!(c == Compact::All(px(2)));
    }

    #[test]
    fn same_output_as_sides() {
        let mut c = Compact::<Corners<Length>>::default();
        let mut s = Corners::default();
        for (c_set, s_set) in [
            (
                Compact::set_tl as fn(&mut _, _),
                Corners::set_tl as fn(&mut _, _),
            ),
            (Compact::set_br, Corners::set_br),
            (Compact::set_t, Corners::set_t),
        ] {
            c_set(&mut c, px(3));
            s_set(&mut s, px(3));
            assert_eq!(Css(&c).to_string(), Css(&s).to_string());
        }
        c.set(px(1));
        s.set(px(1));
        assert_eq!(Css(&c).to_string(), Css(&s).to_string());
    }

    #[test]
    fn merge() {
        let mut a = Compact::<Corners<Length>>::default();
        a.set(px(1));
        let mut b = Compact::default();
        b.set_tl(px(2));
        a.merge_from(b);
        let mut expected = Corners::uniform(px(1));
        expected.set_tl(px(2));
        assert!(a == Compact::Mixed(Box::new(expected)));

        let mut all = Compact::default();
        all.set(px(4));
        a.merge_from(all);
        assert!(a == Compact::All(px(4)));

        a.merge_from(Compact::Unset);
        assert!(a == Compact::All(px(4)));
    }
}
