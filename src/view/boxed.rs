use crate::View;
use crate::context::Context;
use crate::render::Renderer;
use crate::view::RenderFuture;

pub trait BoxedView<'v> {
    fn render(self: Box<Self>, c: &'v Context, r: Renderer) -> RenderFuture<'v>;
}

impl<'v, V: View<'v>> BoxedView<'v> for V {
    fn render(self: Box<Self>, c: &'v Context, r: Renderer) -> RenderFuture<'v> {
        <Self as View>::render(*self, c, r)
    }
}

impl<'v> View<'v> for Box<dyn View<'v>> {
    fn render(self, c: &'v Context, r: Renderer) -> RenderFuture<'v> {
        <dyn View as BoxedView>::render(self, c, r)
    }

    fn boxed(self) -> Box<dyn View<'v>>
    where
        Self: Sized,
    {
        self
    }
}
