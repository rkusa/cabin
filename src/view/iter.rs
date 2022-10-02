use std::fmt;
use std::future::Future;
use std::marker::PhantomData;

use super::IntoView;
pub use super::View;
use crate::render::Renderer;

impl<I, V, M> IntoView<IteratorView<I::IntoIter, V, M>, M> for I
where
    I: IntoIterator<Item = V>,
    I::IntoIter: Send,
    V: View<M> + Send,
    M: Send,
{
    fn into_view(self) -> IteratorView<I::IntoIter, V, M> {
        IteratorView {
            iter: self.into_iter(),
            marker: PhantomData,
        }
    }
}

pub struct IteratorView<I, V, M> {
    iter: I,
    marker: PhantomData<(V, M)>,
}

impl<I, V, M> View<M> for IteratorView<I, V, M>
where
    I: Iterator<Item = V> + Send,
    V: View<M> + Send,
    M: Send,
{
    type Future = impl Future<Output = Result<Renderer, fmt::Error>> + Send;

    fn render(self, mut r: Renderer) -> Self::Future {
        async move {
            for i in self.iter {
                let fut = i.render(r);
                r = fut.await?;
            }
            Ok(r)
        }
    }
}
