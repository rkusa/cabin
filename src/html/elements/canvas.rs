use super::aria::Aria;
use super::common::Common;
use super::global::Global;
use super::input::{Height, Width};
use crate::attribute::WithAttribute;
use crate::context::Context;
use crate::element::Element;

impl Context {
    /// The `canvas` element provides scripts with a resolution-dependent bitmap canvas, which can
    /// be used for rendering graphs, game graphics, art, or other visual images on the fly. The
    /// contents of the canvas element, if any, are the element's fallback content.
    pub fn canvas(&self) -> Element<marker::Canvas> {
        Element::new(self.acquire_renderer(), "canvas")
    }
}

pub mod marker {
    pub struct Canvas;
}

impl Canvas for Element<marker::Canvas> {}
impl Common for Element<marker::Canvas> {}
impl Global for Element<marker::Canvas> {}
impl Aria for Element<marker::Canvas> {}

/// The `canvas` element provides scripts with a resolution-dependent bitmap canvas, which can be
/// used for rendering graphs, game graphics, art, or other visual images on the fly. The contents
/// of the canvas element, if any, are the element's fallback content.
pub trait Canvas: WithAttribute {
    /// Vertical dimension.
    fn height(self, height: u32) -> Self {
        self.with_attribute(Height(height))
    }

    /// Horizontal dimension.
    fn width(self, width: u32) -> Self {
        self.with_attribute(Width(width))
    }
}
