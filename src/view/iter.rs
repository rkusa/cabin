use std::hash::{Hash, Hasher};
use std::iter::Map;
use std::marker::PhantomData;

use twox_hash::XxHash32;

pub use super::View;
use crate::render::Renderer;
use crate::scope::Scope;

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
        f: impl Fn(I::Item) -> B,
    ) -> Map<I, impl FnMut(I::Item) -> KeyedView<B>> {
        self.iter.map(move |item| {
            let key = hash((self.f)(&item));
            Scope::keyed_sync(key, || KeyedView {
                key,
                view: (f)(item),
            })
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

impl<Iter, FV, V> View for Map<Iter, FV>
where
    Iter: Iterator,
    FV: FnMut(Iter::Item) -> V,
    V: View,
{
    async fn render(self, mut r: Renderer, _include_hash: bool) -> Result<Renderer, crate::Error> {
        for i in self {
            let fut = i.render(r, true);
            r = fut.await?;
        }
        Ok(r)
    }

    // TODO: any way to prime without consuming the iterator?
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

impl<V> View for KeyedView<V>
where
    V: View,
{
    async fn render(self, r: Renderer, _include_hash: bool) -> Result<Renderer, crate::Error> {
        let mut el = r.element("cabin-keyed", true)?;
        el.attribute("id", self.key)
            .map_err(crate::error::InternalError::from)?;
        el.content(self.view).await
    }
}
