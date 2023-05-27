use std::future::Future;
use std::marker::PhantomData;
use std::pin::Pin;

use crate::render::Renderer;
use crate::View;

// TODO: any way to reduce this type to one single Box instead of two
type ViewBoxRenderer =
    dyn FnOnce(Renderer) -> Pin<Box<dyn Future<Output = Result<Renderer, crate::Error>>>>;

pub struct BoxedView<Ev> {
    view: Box<ViewBoxRenderer>,
    marker: PhantomData<Ev>,
}

impl<Ev> BoxedView<Ev> {
    pub fn new<V>(view: V) -> Self
    where
        V: View<Ev> + 'static,
        Ev: 'static,
    {
        BoxedView {
            view: Box::new(|r: Renderer| Box::pin(view.render(r))),
            marker: PhantomData,
        }
    }
}

impl<Ev> View<Ev> for BoxedView<Ev> {
    async fn render(self, r: Renderer) -> Result<Renderer, crate::Error> {
        (self.view)(r).await
    }
}
