//! Set the weight (or boldness) of the font (`font-weight`).
//!
//! <https://developer.mozilla.org/en-US/docs/Web/CSS/font-weight>

use crate::Property;

const FONT_FAMILY: &str = "font-family";
const FONT_WEIGHT: &str = "font-weight";

include!(concat!(env!("OUT_DIR"), "/font-family.rs"));

/// ```css
/// font-weight: 100;
/// ```
pub const THIN: Property<u16> = Property(FONT_WEIGHT, 100);

/// ```css
/// font-weight: 200;
/// ```
pub const EXTRALIGHT: Property<u16> = Property(FONT_WEIGHT, 200);

/// ```css
/// font-weight: 300;
/// ```
pub const LIGHT: Property<u16> = Property(FONT_WEIGHT, 300);

/// ```css
/// font-weight: 400;
/// ```
pub const NORMAL: Property<u16> = Property(FONT_WEIGHT, 400);

/// ```css
/// font-weight: 500;
/// ```
pub const MEDIUM: Property<u16> = Property(FONT_WEIGHT, 500);

/// ```css
/// font-weight: 600;
/// ```
pub const SEMIBOLD: Property<u16> = Property(FONT_WEIGHT, 600);

/// ```css
/// font-weight: 700;
/// ```
pub const BOLD: Property<u16> = Property(FONT_WEIGHT, 700);

/// ```css
/// font-weight: 800;
/// ```
pub const EXTRABOLD: Property<u16> = Property(FONT_WEIGHT, 800);

/// ```css
/// font-weight: 900;
/// ```
pub const BLACK: Property<u16> = Property(FONT_WEIGHT, 900);

const FONT_STYLE: &str = "font-style";

/// ```css
/// font-style: italic;
/// ```
pub const ITALIC: Property = Property(FONT_STYLE, "italic");

/// ```css
/// font-style: normal;
/// ```
pub const NOT_ITALIC: Property = Property(FONT_STYLE, "normal");
