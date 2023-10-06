//! Set the tracking (letter spacing) of an element. (`letter-spacing`).
//!
//! <https://developer.mozilla.org/en-US/docs/Web/CSS/letter-spacing>

use crate::{Length, Property};

const LETTER_SPACING: &str = "letter-spacing";

/// ```css
/// letter-spacing: -0.05em;
/// ```
pub const TIGHTER: Property<Length> = Property(LETTER_SPACING, Length::Em(-0.05));

/// ```css
/// letter-spacing: -0.025em;
/// ```
pub const TIGHT: Property<Length> = Property(LETTER_SPACING, Length::Em(-0.025));

/// ```css
/// letter-spacing: 0.0em;
/// ```
pub const NORMAL: Property<Length> = Property(LETTER_SPACING, Length::Em(0.0));

/// ```css
/// letter-spacing: 0.025em;
/// ```
pub const WIDE: Property<Length> = Property(LETTER_SPACING, Length::Em(0.025));

/// ```css
/// letter-spacing: 0.05em;
/// ```
pub const WIDER: Property<Length> = Property(LETTER_SPACING, Length::Em(0.05));

/// ```css
/// letter-spacing: 0.1em;
/// ```
pub const WIDEST: Property<Length> = Property(LETTER_SPACING, Length::Em(0.1));
