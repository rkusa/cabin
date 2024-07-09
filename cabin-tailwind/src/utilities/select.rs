//! Set whether an element's text can be selected (`user-select`).
//!
//! <https://developer.mozilla.org/en-US/docs/Web/CSS/user-select>

use crate::PropertyTwice;

const USER_SELECT: &str = "user-select";
const __WEBKIT_USER_SELECT: &str = "-webkit-user-select";

/// ```css
/// user-select: none;
/// ```
pub const NONE: PropertyTwice = PropertyTwice(USER_SELECT, __WEBKIT_USER_SELECT, "none");

/// ```css
/// user-select: text;
/// ```
pub const TEXT: PropertyTwice = PropertyTwice(USER_SELECT, __WEBKIT_USER_SELECT, "text");

/// ```css
/// user-select: all;
/// ```
pub const ALL: PropertyTwice = PropertyTwice(USER_SELECT, __WEBKIT_USER_SELECT, "all");

/// ```css
/// user-select: auto;
/// ```
pub const AUTO: PropertyTwice = PropertyTwice(USER_SELECT, __WEBKIT_USER_SELECT, "auto");
