use super::common::Common;
use super::global::Global;
use super::input::{Height, Width};
use crate::View;
use crate::html::attributes::{Attributes, WithAttribute};
use crate::html::{Aria, Html};

/// The `canvas` element provides scripts with a resolution-dependent bitmap canvas, which can be
/// used for rendering graphs, game graphics, art, or other visual images on the fly. The contents
/// of the canvas element, if any, are the element's fallback content.
#[crate::view_macro(crate::html::elements::canvas)]
pub fn canvas(content: impl View) -> Html<marker::Canvas, ()> {
    Html::new("canvas", (), content)
}

pub mod marker {
    pub struct Canvas;
}

impl<A: Attributes> Canvas for Html<marker::Canvas, A> {}
impl<A: Attributes> Common for Html<marker::Canvas, A> {}
impl<A: Attributes> Global for Html<marker::Canvas, A> {}
impl<A: Attributes> Aria for Html<marker::Canvas, A> {}

/// The `canvas` element provides scripts with a resolution-dependent bitmap canvas, which can be
/// used for rendering graphs, game graphics, art, or other visual images on the fly. The contents
/// of the canvas element, if any, are the element's fallback content.
pub trait Canvas: WithAttribute {
    /// Vertical dimension.
    fn height(self, height: u32) -> Self::Output<Height> {
        self.with_attribute(Height(height))
    }

    /// Horizontal dimension.
    fn width(self, width: u32) -> Self::Output<Width> {
        self.with_attribute(Width(width))
    }
}
