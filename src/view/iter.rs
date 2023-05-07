use std::future::Future;
use std::hash::{Hash, Hasher};
use std::marker::PhantomData;
use std::pin::Pin;

use super::IntoView;
pub use super::View;
use crate::render::Renderer;
use twox_hash::XxHash32;

pub struct KeyedView<V> {
    key: u32,
    view: V,
}

impl<V> KeyedView<V> {
    pub(crate) fn new(key: impl Hash, view: V) -> Self {
        let mut hasher = XxHash32::default();
        key.hash(&mut hasher);
        Self {
            key: hasher.finish() as u32,
            view,
        }
    }
}

impl<Iter, I, V> IntoView<IteratorView<Iter::IntoIter, I, V>> for Iter
where
    Iter: IntoIterator<Item = KeyedView<I>>,
    // TODO: remove `+ 'static` once removing away from boxed future
    Iter::IntoIter: Send + 'static,
    I: IntoView<V> + Send,
    V: View + Send,
{
    fn into_view(self) -> IteratorView<Iter::IntoIter, I, V> {
        IteratorView {
            iter: self.into_iter(),
            marker: PhantomData,
        }
    }
}

pub struct IteratorView<Iter, I, V> {
    iter: Iter,
    marker: PhantomData<(I, V)>,
}

impl<Iter, I, V> View for IteratorView<Iter, I, V>
where
    // TODO: remove `+ 'static` once removing away from boxed future
    Iter: Iterator<Item = KeyedView<I>> + Send + 'static,
    I: IntoView<V> + Send,
    V: View + Send,
{
    // TODO: move to `impl Future` once `type_alias_impl_trait` is stable
    type Future = Pin<Box<dyn Future<Output = Result<Renderer, crate::Error>> + Send>>;

    fn render(self, mut r: Renderer) -> Self::Future {
        Box::pin(async move {
            for i in self.iter {
                let fut = i.view.into_view().render(r);
                r = fut.await?;
            }
            Ok(r)
        })
    }
}
