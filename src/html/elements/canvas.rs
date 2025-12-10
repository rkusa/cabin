use super::common::Common;
use super::global::Global;
use super::input::{Height, Width};
use crate::View;
use crate::html::attributes::{Attributes, WithAttribute};
use crate::html::{Aria, Html};

/// The `canvas` element provides scripts with a resolution-dependent bitmap canvas, which can be
/// used for rendering graphs, game graphics, art, or other visual images on the fly. The contents
/// of the canvas element, if any, are the element's fallback content.
pub fn canvas(content: impl View) -> Html<marker::Canvas, ()> {
    #[cfg(debug_assertions)]
    let content = content.boxed();
    Html::new("canvas", (), content)
}

mod macros {
    #[macro_export]
    macro_rules! canvas {
        ($($x:tt)*) => {
            $crate::html::elements::canvas::canvas($crate::view![$($x)*])
        }
    }

    pub use canvas;
}

pub use macros::canvas;

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
