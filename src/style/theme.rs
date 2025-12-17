use crate::style::collector::StyleDelegate;
use crate::style::units::float::Float;
use crate::style::units::length::Length;
use crate::style::{Style, SubStyle};

include!(concat!(env!("OUT_DIR"), "/theme.rs"));

impl<T> ThemeExt for T where T: Style {}
impl<T> ThemeSubExt for T where T: SubStyle {}
