use std::future::Future;
use std::pin::Pin;

use crate::render::Renderer;
use crate::View;

// TODO: any way to reduce this type to one single Box instead of two
type ViewBoxRenderer =
    dyn FnOnce(Renderer, bool) -> Pin<Box<dyn Future<Output = Result<Renderer, crate::Error>>>>;

pub struct BoxedView {
    view: Box<ViewBoxRenderer>,
}

impl BoxedView {
    pub fn new<V>(view: V) -> Self
    where
        V: View + 'static,
    {
        BoxedView {
            view: Box::new(|r: Renderer, include_hash: bool| {
                Box::pin(view.render(r, include_hash))
            }),
        }
    }
}

impl View for BoxedView {
    async fn render(self, r: Renderer, include_hash: bool) -> Result<Renderer, crate::Error> {
        (self.view)(r, include_hash).await
    }

    // TODO: prime
}
