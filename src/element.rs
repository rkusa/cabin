use std::marker::PhantomData;

use crate::View;
use crate::attribute::{Attribute, WithAttribute};
use crate::context::Context;
use crate::fragment::Fragment;
use crate::render::Renderer;
use crate::view::{IntoView, RenderFuture};

pub struct Element<'v, El> {
    tag: &'static str,
    renderer: Renderer,
    context: &'v Context,
    hash_offset: Option<usize>,
    error: Option<crate::Error>,
    marker: PhantomData<El>,
}

impl<'v, El> Element<'v, El> {
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

    pub fn child(mut self, child: impl IntoView<'v>) -> ElementContent<'v> {
        if let Some(err) = self.error {
            return ElementContent(ElementContentState::Error(err));
        }

        self.renderer.start_content();
        let fragment = Fragment::new(self.renderer, self.context).child(child);
        ElementContent(ElementContentState::Content {
            tag: self.tag,
            hash_offset: self.hash_offset,
            fragment,
        })
    }
}

pub struct ElementContent<'v>(ElementContentState<'v>);

enum ElementContentState<'v> {
    Content {
        tag: &'static str,
        hash_offset: Option<usize>,
        fragment: Fragment<'v>,
    },
    Error(crate::Error),
}

impl<'v> ElementContent<'v> {
    pub fn child(self, child: impl IntoView<'v>) -> Self {
        ElementContent(match self.0 {
            ElementContentState::Content {
                tag,
                hash_offset,
                fragment,
            } => ElementContentState::Content {
                tag,
                hash_offset,
                fragment: fragment.child(child),
            },
            ElementContentState::Error(err) => ElementContentState::Error(err),
        })
    }
}

impl<'v, El: 'v> View<'v> for Element<'v, El> {
    fn render(mut self, mut r: Renderer) -> RenderFuture<'v> {
        if let Some(err) = self.error {
            return RenderFuture::ready(Err(err));
        }

        self.renderer.start_content();
        self.renderer.end_element(self.tag, false, self.hash_offset);
        r.append(&mut self.renderer);
        self.context.release_renderer(self.renderer);
        RenderFuture::ready(Ok(r))
    }
}

impl<'v> View<'v> for ElementContent<'v> {
    fn render(self, r: Renderer) -> RenderFuture<'v> {
        let offset = r.len();
        match self.0 {
            ElementContentState::Content {
                tag,
                hash_offset,
                fragment,
            } => match fragment.render(r) {
                RenderFuture::Ready(Some(Ok(mut r))) => {
                    r.end_element(tag, false, hash_offset.map(|o| o + offset));
                    RenderFuture::ready(Ok(r))
                }
                RenderFuture::Ready(Some(Err(err))) => RenderFuture::Ready(Some(Err(err))),
                RenderFuture::Ready(None) => RenderFuture::Ready(None),
                RenderFuture::Future(future) => RenderFuture::Future(Box::pin(async move {
                    match future.await {
                        Ok(mut r) => {
                            r.end_element(tag, false, hash_offset.map(|o| o + offset));
                            Ok(r)
                        }
                        Err(err) => Err(err),
                    }
                })),
            },
            ElementContentState::Error(err) => RenderFuture::ready(Err(err)),
        }
    }
}

impl<'v, El> WithAttribute for Element<'v, El> {
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
