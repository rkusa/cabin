use crate::style::Style;
use crate::style::units::length::Length;

include!(concat!(env!("OUT_DIR"), "/theme.rs"));

impl<T> ThemeExt for T where T: Style {}
