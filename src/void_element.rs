use std::marker::PhantomData;

use crate::View;
use crate::attribute::{Attribute, WithAttribute};
use crate::render::Renderer;
use crate::view::internal::Internal;

pub struct VoidElement<El>(Internal<VoidElementBuilder<El>>);

struct VoidElementBuilder<El> {
    tag: &'static str,
    renderer: Renderer,
    hash_offset: Option<usize>,
    marker: PhantomData<El>,
}

impl<El> VoidElement<El> {
    pub fn new(mut renderer: Renderer, tag: &'static str) -> Self {
        let hash_offset = renderer.start_element(tag);

        Self(Internal::new(VoidElementBuilder {
            tag,
            renderer,
            hash_offset,
            marker: PhantomData,
        }))
    }

    fn render(self) -> Result<Renderer, crate::Error> {
        let mut builder = self.0.take_builder()?;
        builder
            .renderer
            .end_element(builder.tag, true, builder.hash_offset);
        Ok(builder.renderer)
    }
}

impl<El> View for VoidElement<El> {
    fn render(self, r: &mut Renderer) -> Result<(), crate::Error> {
        r.append(self.render()?);
        Ok(())
    }
}

impl<El> WithAttribute for VoidElement<El> {
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
