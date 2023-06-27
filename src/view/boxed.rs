use super::RenderFuture;
use crate::render::Renderer;
use crate::View;

// FIXME: any way to reduce this type to one single Box instead of two
type ViewBoxRenderer = dyn FnOnce(Renderer, bool) -> RenderFuture;

pub struct BoxedView {
    view: Box<ViewBoxRenderer>,
}

impl BoxedView {
    pub fn new<V>(view: V) -> Self
    where
        V: View + 'static,
    {
        BoxedView {
            view: Box::new(|r: Renderer, include_hash: bool| view.render(r, include_hash)),
        }
    }
}

impl View for BoxedView {
    fn render(self, r: Renderer, include_hash: bool) -> RenderFuture {
        (self.view)(r, include_hash)
    }

    // FIXME: prime
}
