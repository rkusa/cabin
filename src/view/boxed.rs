use std::future::Future;
use std::pin::Pin;

use crate::render::Renderer;
use crate::View;

// TODO: any way to reduce this type to one single Box instead of two
type ViewBoxRenderer = dyn FnOnce(
        Renderer,
    ) -> Pin<Box<dyn Future<Output = Result<Renderer, crate::Error>> + Send + 'static>>
    + Send;

pub struct BoxedView {
    view: Box<ViewBoxRenderer>,
}

impl BoxedView {
    pub fn new<V>(view: V) -> Self
    where
        V: View + Send + 'static,
        V::Future: 'static,
    {
        BoxedView {
            view: Box::new(|r: Renderer| Box::pin(view.render(r))),
        }
    }
}

impl View for BoxedView {
    type Future = Pin<Box<dyn Future<Output = Result<Renderer, crate::Error>> + Send + 'static>>;

    fn render(self, r: Renderer) -> Self::Future {
        (self.view)(r)
    }
}
