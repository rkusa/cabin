use std::fmt;

use crate::Utility;

pub struct DurationMs(pub u32);

/// Duration of CSS animations in milliseconds.
/// ```css
/// animation-duration: {ms}ms;
/// ```
pub fn duration_ms(ms: u32) -> DurationMs {
    DurationMs(ms)
}

impl Utility for DurationMs {
    fn declarations(&self, f: &mut dyn fmt::Write) -> fmt::Result {
        write!(f, "animation-duration: {}ms;", self.0)?;
        Ok(())
    }

    fn order(&self) -> usize {
        1
    }
}

pub struct DurationS(pub f32);

/// Duration of CSS animations in seconds.
/// ```css
/// animation-duration: {s}s;
/// ```
pub fn duration_s(s: f32) -> DurationS {
    DurationS(s)
}

impl Utility for DurationS {
    fn declarations(&self, f: &mut dyn fmt::Write) -> fmt::Result {
        write!(f, "animation-duration: {}s;", self.0)?;
        Ok(())
    }

    fn order(&self) -> usize {
        1
    }
}

pub struct DelayMs(pub u32);

/// Delay of CSS animations in milliseconds.
/// ```css
/// animation-delay: {ms}ms;
/// ```
pub fn delay_ms(ms: u32) -> DelayMs {
    DelayMs(ms)
}

impl Utility for DelayMs {
    fn declarations(&self, f: &mut dyn fmt::Write) -> fmt::Result {
        write!(f, "animation-delay: {}ms;", self.0)?;
        Ok(())
    }

    fn order(&self) -> usize {
        1
    }
}

pub struct DelayS(pub f32);

/// Delay of CSS animations in seconds.
/// ```css
/// animation-delay: {s}s;
/// ```
pub fn delay_s(s: f32) -> DelayS {
    DelayS(s)
}

impl Utility for DelayS {
    fn declarations(&self, f: &mut dyn fmt::Write) -> fmt::Result {
        write!(f, "animation-delay: {}s;", self.0)?;
        Ok(())
    }

    fn order(&self) -> usize {
        1
    }
}

pub struct IterationCount(pub u16);

/// Number of times the animation is played before stopping.
/// ```css
/// animation-iteration-count: {n};
/// ```
pub fn iterations(n: u16) -> IterationCount {
    IterationCount(n)
}

impl Utility for IterationCount {
    fn declarations(&self, f: &mut dyn fmt::Write) -> fmt::Result {
        write!(f, "animation-iteration-count: {};", self.0)?;
        Ok(())
    }

    fn order(&self) -> usize {
        1
    }
}

pub struct Infinite;

/// Run the animation indefinitely.
/// ```css
/// animation-iteration-count: infinite;
/// ```
pub const INFINITE: Infinite = Infinite;

impl Utility for Infinite {
    fn declarations(&self, f: &mut dyn fmt::Write) -> fmt::Result {
        write!(f, "animation-iteration-count: infinite;")?;
        Ok(())
    }

    fn order(&self) -> usize {
        1
    }
}
