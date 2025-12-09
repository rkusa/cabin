use std::hash::{Hash, Hasher};
use std::iter::{FilterMap, Map};
use std::marker::PhantomData;

use twox_hash::XxHash32;

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

impl<Iter, FV, V> View for Map<Iter, FV>
where
    Iter: Iterator,
    FV: FnMut(Iter::Item) -> V,
    V: View,
{
    fn render(self, r: &mut Renderer) -> Result<(), crate::Error> {
        for i in self {
            i.render(r)?;
        }
        Ok(())
    }
}

impl<Iter, FV, V> View for FilterMap<Iter, FV>
where
    Iter: Iterator + Send,
    FV: FnMut(Iter::Item) -> Option<V> + Send,
    V: View,
{
    fn render(self, r: &mut Renderer) -> Result<(), crate::Error> {
        for i in self {
            i.render(r)?;
        }
        Ok(())
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

impl<V> View for KeyedView<V>
where
    V: View,
{
    fn render(self, r: &mut Renderer) -> Result<(), crate::Error> {
        let parent_hasher = r.take_hasher();
        let hash_offset = r.start_element("cabin-keyed");
        r.attribute("id", self.key);
        r.start_content();

        self.view.render(r)?;
        r.end_element("cabin-keyed", false, hash_offset);
        r.merge_hasher(parent_hasher);
        Ok(())
    }
}

impl<F, V> Future for KeyedView<F>
where
    F: Future<Output = V>,
    V: View + Unpin,
{
    type Output = KeyedView<V>;

    fn poll(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        let key = self.as_ref().key;
        // FIXME: check soundness or get rid of unsafe
        let view = unsafe { self.map_unchecked_mut(|v| &mut v.view) };
        match view.poll(cx) {
            std::task::Poll::Ready(view) => std::task::Poll::Ready(KeyedView { key, view }),
            std::task::Poll::Pending => std::task::Poll::Pending,
        }
    }
}
