use std::fmt;

use crate::style::{Style, StyleDefinition};

#[derive(Default)]
pub struct AnimationStyle(Box<StyleDefinition>);

impl Style for AnimationStyle {
    fn style_mut(&mut self) -> &mut StyleDefinition {
        &mut self.0
    }
}

impl fmt::Display for AnimationStyle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}
