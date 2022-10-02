use std::fmt;
use std::future::Future;
use std::marker::PhantomData;
use std::pin::Pin;

use crate::render::Renderer;
use crate::View;

// TODO: any way to reduce this type to one single Box instead of two
type ViewBoxRenderer = dyn FnOnce(Renderer) -> Pin<Box<dyn Future<Output = Result<Renderer, fmt::Error>> + Send + 'static>>
    + Send;

pub struct BoxedView<M> {
    view: Box<ViewBoxRenderer>,
    marker: PhantomData<M>,
}

impl<M> BoxedView<M> {
    pub fn new<V>(view: V) -> Self
    where
        V: View<M> + Send + 'static,
        V::Future: 'static,
    {
        BoxedView {
            view: Box::new(|r: Renderer| Box::pin(view.render(r))),
            marker: PhantomData,
        }
    }
}

impl<M> View<M> for BoxedView<M> {
    type Future = Pin<Box<dyn Future<Output = Result<Renderer, fmt::Error>> + Send + 'static>>;

    fn render(self, r: Renderer) -> Self::Future {
        (self.view)(r)
    }
}
