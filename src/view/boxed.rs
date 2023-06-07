use std::future::Future;
use std::marker::PhantomData;
use std::pin::Pin;

use crate::render::Renderer;
use crate::View;

// TODO: any way to reduce this type to one single Box instead of two
type ViewBoxRenderer =
    dyn FnOnce(Renderer) -> Pin<Box<dyn Future<Output = Result<Renderer, crate::Error>>>>;

pub struct BoxedView {
    view: Box<ViewBoxRenderer>,
    marker: PhantomData,
}

impl BoxedView {
    pub fn new<V>(view: V) -> Self
    where
        V: View + 'static,
        Ev: 'static,
    {
        BoxedView {
            view: Box::new(|r: Renderer| Box::pin(view.render(r))),
            marker: PhantomData,
        }
    }
}

impl View for BoxedView {
    async fn render(self, r: Renderer, include_hash: bool) -> Result<Renderer, crate::Error> {
        (self.view)(r).await
    }

    // TODO: prime
}
