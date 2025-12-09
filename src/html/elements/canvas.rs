use super::aria::Aria;
use super::common::Common;
use super::global::Global;
use super::input::{Height, Width};
use crate::attribute::WithAttribute;
use crate::element::{Element, ElementProxy};

/// The `canvas` element provides scripts with a resolution-dependent bitmap canvas, which can
/// be used for rendering graphs, game graphics, art, or other visual images on the fly. The
/// contents of the canvas element, if any, are the element's fallback content.
pub fn canvas() -> Element<marker::Canvas> {
    Element::new("canvas")
}

pub mod marker {
    pub struct Canvas;

    impl<'v, V: crate::View + 'v> crate::element::IntoChild<'v, Canvas> for V {
        fn into_child(self) -> impl crate::View {
            self
        }
    }
}

impl<E, P> Canvas<(marker::Canvas, P)> for E where E: ElementProxy<marker::Canvas, P> {}
impl<E, P> Common<(marker::Canvas, P)> for E where E: ElementProxy<marker::Canvas, P> {}
impl<E, P> Global<(marker::Canvas, P)> for E where E: ElementProxy<marker::Canvas, P> {}
impl<E, P> Aria<(marker::Canvas, P)> for E where E: ElementProxy<marker::Canvas, P> {}

/// The `canvas` element provides scripts with a resolution-dependent bitmap canvas, which can be
/// used for rendering graphs, game graphics, art, or other visual images on the fly. The contents
/// of the canvas element, if any, are the element's fallback content.
pub trait Canvas<T>: WithAttribute {
    /// Vertical dimension.
    fn height(self, height: u32) -> Self {
        self.with_attribute(Height(height))
    }

    /// Horizontal dimension.
    fn width(self, width: u32) -> Self {
        self.with_attribute(Width(width))
    }
}
