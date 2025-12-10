use super::RenderFuture;
use crate::View;
use crate::render::Renderer;

type ViewBoxRenderer = dyn FnOnce(Renderer) -> RenderFuture + Send;

pub struct BoxedView {
    view: Box<ViewBoxRenderer>,
}

impl BoxedView {
    pub fn new<V>(view: V) -> Self
    where
        V: View,
    {
        BoxedView {
            view: Box::new(|r: Renderer| view.render(r)),
        }
    }
}

impl View for BoxedView {
    fn render(self, r: Renderer) -> RenderFuture {
        (self.view)(r)
    }

    // TODO: prime
}
