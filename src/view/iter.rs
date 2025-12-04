use std::hash::{Hash, Hasher};
use std::iter::{FilterMap, Map};
use std::marker::PhantomData;

use twox_hash::XxHash32;

use super::RenderFuture;
pub use super::View;
use crate::render::Renderer;

pub trait IteratorExt
where
    Self: Iterator,
{
    fn keyed<F, K>(self, f: F) -> Keyed<Self, F, K>
    where
        Self: Sized,
        F: FnMut(&Self::Item) -> K,
        K: Hash;
}

impl<Iter> IteratorExt for Iter
where
    Iter: Iterator,
{
    fn keyed<F, K>(self, f: F) -> Keyed<Iter, F, K>
    where
        Iter: Iterator,
        F: FnMut(&Iter::Item) -> K,
        K: Hash,
    {
        Keyed::new(self, f)
    }
}

pub struct Keyed<I, F, K> {
    iter: I,
    f: F,
    marker: PhantomData<K>,
}

impl<I, F, K> Keyed<I, F, K>
where
    I: Iterator,
    F: FnMut(&I::Item) -> K,
    K: Hash,
{
    pub fn new(iter: I, f: F) -> Self {
        Self {
            iter,
            f,
            marker: PhantomData,
        }
    }

    pub fn map<B>(
        mut self,
        mut f: impl FnMut(I::Item) -> B,
    ) -> Map<I, impl FnMut(I::Item) -> KeyedView<B>> {
        self.iter.map(move |item| {
            let key = hash((self.f)(&item));
            KeyedView {
                key,
                view: (f)(item),
            }
        })
    }
}

impl<I: Iterator, F, K> Iterator for Keyed<I, F, K>
where
    F: FnMut(&I::Item) -> K,
    K: Hash,
{
    type Item = I::Item;

    #[inline]
    fn next(&mut self) -> Option<I::Item> {
        self.iter.next()
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

impl<'v, Iter, FV, V> View<'v> for Map<Iter, FV>
where
    Iter: Iterator + 'v,
    FV: FnMut(Iter::Item) -> V + 'v,
    V: View<'v>,
{
    fn render(mut self, mut r: Renderer) -> RenderFuture<'v> {
        while let Some(i) = self.next() {
            match i.render(r) {
                RenderFuture::Ready(Some(Ok(renderer))) => r = renderer,
                RenderFuture::Ready(Some(Err(err))) => return RenderFuture::Ready(Some(Err(err))),
                RenderFuture::Ready(None) => return RenderFuture::Ready(None),
                // Only return future upon the first future item is encountered
                RenderFuture::Future(future) => {
                    return RenderFuture::Future(Box::pin(async move {
                        let mut r = future.await?;
                        for i in self {
                            r = i.render(r).await?;
                        }
                        Ok(r)
                    }));
                }
            }
        }
        RenderFuture::Ready(Some(Ok(r)))
    }
}

impl<'v, Iter, FV, V> View<'v> for FilterMap<Iter, FV>
where
    Iter: Iterator + Send + 'v,
    FV: FnMut(Iter::Item) -> Option<V> + Send + 'v,
    V: View<'v>,
{
    fn render(mut self, mut r: Renderer) -> RenderFuture<'v> {
        while let Some(i) = self.next() {
            match i.render(r) {
                RenderFuture::Ready(Some(Ok(renderer))) => r = renderer,
                RenderFuture::Ready(Some(Err(err))) => return RenderFuture::Ready(Some(Err(err))),
                RenderFuture::Ready(None) => return RenderFuture::Ready(None),
                // Only return future upon the first future item is encountered
                RenderFuture::Future(future) => {
                    return RenderFuture::Future(Box::pin(async move {
                        let mut r = future.await?;
                        for i in self {
                            r = i.render(r).await?;
                        }
                        Ok(r)
                    }));
                }
            }
        }
        RenderFuture::Ready(Some(Ok(r)))
    }
}

fn hash(val: impl Hash) -> u32 {
    let mut hasher = XxHash32::default();
    val.hash(&mut hasher);
    hasher.finish() as u32
}

pub struct KeyedView<V> {
    key: u32,
    view: V,
}

impl<'v, V> View<'v> for KeyedView<V>
where
    V: View<'v>,
{
    fn render(self, mut r: Renderer) -> RenderFuture<'v> {
        let hash_offset = r.start_element("cabin-keyed");
        r.attribute("id", self.key);
        r.start_content();

        match self.view.render(r) {
            RenderFuture::Ready(Some(Ok(mut r))) => {
                r.end_element("cabin-keyed", false, hash_offset);
                RenderFuture::ready(Ok(r))
            }
            rf @ RenderFuture::Ready(_) => rf,
            RenderFuture::Future(future) => RenderFuture::Future(Box::pin(async move {
                match future.await {
                    Ok(mut r) => {
                        r.end_element("cabin-keyed", false, hash_offset);
                        Ok(r)
                    }
                    Err(err) => Err(err),
                }
            })),
        }
    }
}
