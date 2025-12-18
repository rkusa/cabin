use std::fmt::{self, Write as _};
use std::ops::Mul;

/// Three-places float.
#[derive(Default, Clone, Copy, Hash, PartialEq, Eq)]
pub struct Float(i32);

impl Float {
    pub fn is_zero(&self) -> bool {
        self.0 == 0
    }

    pub fn to_f32(&self) -> f32 {
        (self.0 as f32) / 1000.0
    }
}

impl From<f32> for Float {
    fn from(value: f32) -> Self {
        // FIXME: handle overflow?
        Self((value * 1000.0).round() as i32)
    }
}

impl From<i32> for Float {
    fn from(value: i32) -> Self {
        Self(value * 1000)
    }
}

impl From<i16> for Float {
    fn from(value: i16) -> Self {
        Self(value as i32 * 1000)
    }
}

impl Mul<f32> for Float {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self::from(self.to_f32() * rhs)
    }
}

impl fmt::Display for Float {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let l = self.0 / 1000;
        let r = (self.0 % 1000).abs();
        if l == 0 && self.0 < 0 {
            write!(f, "-")?;
        }
        if r == 0 {
            write!(f, "{l}")
        } else {
            write!(f, "{l}.")?;
            write!(
                SkipTrailingZeroes {
                    wr: f,
                    trim_zeroes: false
                },
                "{r:<03}"
            )
        }
    }
}

struct SkipTrailingZeroes<'a> {
    wr: &'a mut dyn fmt::Write,
    trim_zeroes: bool,
}

impl<'a> fmt::Write for SkipTrailingZeroes<'a> {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        if s.chars().next().is_some_and(|c| c != '0') {
            self.trim_zeroes = true;
        }
        if self.trim_zeroes {
            self.wr.write_str(s.trim_end_matches('0'))
        } else {
            self.wr.write_str(s)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display() {
        assert_eq!(Float::from(1.0).to_string(), "1");
        assert_eq!(Float::from(1.2).to_string(), "1.2");
        assert_eq!(Float::from(1.03).to_string(), "1.03");
        assert_eq!(Float::from(1.004).to_string(), "1.004");
        assert_eq!(Float::from(1.0006).to_string(), "1.001");
        assert_eq!(Float::from(1.00006).to_string(), "1");
        assert_eq!(Float::from(-1.0).to_string(), "-1");
        assert_eq!(Float::from(-1.2).to_string(), "-1.2");
        assert_eq!(Float::from(-0.5).to_string(), "-0.5");
    }
}
