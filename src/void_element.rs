use std::marker::PhantomData;

use crate::View;
use crate::attribute::{Attribute, WithAttribute};
use crate::context::Context;
use crate::render::Renderer;
use crate::view::RenderFuture;
use crate::view::internal::{Internal, Render};

pub struct VoidElement<'v, El>(Internal<'v, VoidElementBuilder<'v, El>>);

struct VoidElementBuilder<'v, El> {
    tag: &'static str,
    renderer: Renderer,
    hash_offset: Option<usize>,
    marker: PhantomData<&'v El>,
}

impl<'v, El> VoidElement<'v, El> {
    pub fn new(context: &'v Context, tag: &'static str) -> Self {
        let mut r = context.acquire_renderer();
        let hash_offset = r.start_element(tag);

        Self(Internal::new(VoidElementBuilder {
            tag,
            renderer: r,
            hash_offset,
            marker: PhantomData,
        }))
    }
}

impl<'v, El> Render<'v> for VoidElementBuilder<'v, El> {
    fn render(mut self) -> RenderFuture<'v> {
        self.renderer.end_element(self.tag, true, self.hash_offset);
        RenderFuture::ready(Ok(self.renderer))
    }
}

impl<'v, El: 'v> View<'v> for VoidElement<'v, El> {
    fn render(self, c: &'v Context, r: Renderer) -> RenderFuture<'v> {
        self.0.render().merge_into(c, r)
    }
}

impl<'v, El> WithAttribute for VoidElement<'v, El> {
    fn with_attribute(mut self, attr: impl Attribute) -> Self {
        let Some(builder) = self.0.builder_mut() else {
            return self;
        };

        if let Err(err) = attr.render(&mut builder.renderer) {
            self.0.errored(err);
        }

        self
    }
}
