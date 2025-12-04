use std::marker::PhantomData;

use crate::View;
use crate::attribute::{Attribute, WithAttribute};
use crate::context::Context;
use crate::render::Renderer;
use crate::view::RenderFuture;

pub struct VoidElement<'v, El> {
    tag: &'static str,
    renderer: Renderer,
    context: &'v Context,
    hash_offset: Option<usize>,
    error: Option<crate::Error>,
    marker: PhantomData<El>,
}

impl<'v, El> VoidElement<'v, El> {
    pub fn new(context: &'v Context, tag: &'static str) -> Self {
        let mut r = context.acquire_renderer();
        let hash_offset = r.start_element(tag);

        Self {
            tag,
            renderer: r,
            context,
            hash_offset,
            error: None,
            marker: PhantomData,
        }
    }
}

impl<'v, El: 'v> View<'v> for VoidElement<'v, El> {
    fn render(mut self, mut r: Renderer) -> RenderFuture<'v> {
        if let Some(err) = self.error {
            return RenderFuture::ready(Err(err));
        }

        self.renderer.end_element(self.tag, true, self.hash_offset);
        r.append(&mut self.renderer);
        self.context.release_renderer(self.renderer);
        RenderFuture::ready(Ok(r))
    }
}

impl<'v, El> WithAttribute for VoidElement<'v, El> {
    fn with_attribute(mut self, attr: impl Attribute) -> Self {
        if self.error.is_some() {
            return self;
        }
        if let Err(err) = attr.render(&mut self.renderer) {
            self.error = Some(err);
        }
        self
    }
}
