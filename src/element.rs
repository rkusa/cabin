use std::marker::PhantomData;

use crate::View;
use crate::attribute::{Attribute, WithAttribute};
use crate::context::Context;
use crate::fragment::Fragment;
use crate::render::Renderer;
use crate::view::internal::{Internal, Render};
use crate::view::{IntoView, RenderFuture};

pub struct Element<'v, El>(Internal<'v, ElementBuilder<'v, El>>);

struct ElementBuilder<'v, El> {
    tag: &'static str,
    renderer: Renderer,
    context: &'v Context,
    hash_offset: Option<usize>,
    marker: PhantomData<El>,
}

impl<'v, El> Element<'v, El> {
    pub fn new(context: &'v Context, tag: &'static str) -> Self {
        let mut r = context.acquire_renderer();
        let hash_offset = r.start_element(tag);

        Self(Internal::new(ElementBuilder {
            tag,
            renderer: r,
            context,
            hash_offset,
            marker: PhantomData,
        }))
    }

    pub fn child(self, child: impl IntoView<'v>) -> ElementContent<'v> {
        let mut el = match self.0.take_builder() {
            Ok(builder) => builder,
            Err(err) => return ElementContent(Internal::error(err)),
        };

        el.renderer.start_content();
        let fragment = Fragment::new(el.renderer, el.context).child(child);
        ElementContent(Internal::new(ElementContentBuilder {
            tag: el.tag,
            hash_offset: el.hash_offset,
            fragment,
        }))
    }
}

impl<'v, El> Render<'v> for ElementBuilder<'v, El> {
    fn render(mut self) -> RenderFuture<'v> {
        self.renderer.start_content();
        self.renderer.end_element(self.tag, false, self.hash_offset);
        RenderFuture::ready(Ok(self.renderer))
    }
}

impl<'v, El: 'v> View<'v> for Element<'v, El> {
    fn render(self, c: &'v Context, r: Renderer) -> RenderFuture<'v> {
        self.0.render().merge_into(c, r)
    }
}

impl<'v, El> WithAttribute for Element<'v, El> {
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

pub struct ElementContent<'v>(Internal<'v, ElementContentBuilder<'v>>);

struct ElementContentBuilder<'v> {
    tag: &'static str,
    hash_offset: Option<usize>,
    fragment: Fragment<'v>,
}

impl<'v> ElementContent<'v> {
    pub fn child(mut self, child: impl IntoView<'v>) -> Self {
        let Some(builder) = self.0.builder_mut() else {
            return self;
        };

        builder.fragment.append_child(child);

        self
    }
}

impl<'v> Render<'v> for ElementContentBuilder<'v> {
    fn render(self) -> RenderFuture<'v> {
        match self.fragment.render() {
            RenderFuture::Ready(Some(Ok(mut r))) => {
                r.end_element(self.tag, false, self.hash_offset);
                RenderFuture::ready(Ok(r))
            }
            RenderFuture::Ready(Some(Err(err))) => RenderFuture::Ready(Some(Err(err))),
            RenderFuture::Ready(None) => RenderFuture::Ready(None),
            RenderFuture::Future(future) => RenderFuture::Future(Box::pin(async move {
                match future.await {
                    Ok(mut r) => {
                        r.end_element(self.tag, false, self.hash_offset);
                        Ok(r)
                    }
                    Err(err) => Err(err),
                }
            })),
        }
    }
}

impl<'v> View<'v> for ElementContent<'v> {
    fn render(self, c: &'v Context, r: Renderer) -> RenderFuture<'v> {
        self.0.render().merge_into(c, r)
    }
}
