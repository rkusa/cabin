use super::RenderFuture;
use crate::render::Renderer;
use crate::View;

type ViewBoxRenderer = dyn FnOnce(Renderer, bool) -> RenderFuture + Send;

pub struct BoxedView {
    view: Box<ViewBoxRenderer>,
}

impl BoxedView {
    pub fn new<V>(view: V) -> Self
    where
        V: View,
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

    // TODO: prime
}
