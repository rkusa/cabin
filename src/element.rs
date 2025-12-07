use std::marker::PhantomData;

use crate::View;
use crate::attribute::{Attribute, WithAttribute};
use crate::fragment::Fragment;
use crate::render::Renderer;
use crate::view::AnyView;
use crate::view::any::IntoAnyView;
use crate::view::internal::Internal;

pub struct Element<El>(Internal<ElementBuilder<El>>);

struct ElementBuilder<El> {
    tag: &'static str,
    renderer: Renderer,
    hash_offset: Option<usize>,
    marker: PhantomData<El>,
}

impl<El> Element<El> {
    pub fn new(mut renderer: Renderer, tag: &'static str) -> Self {
        let hash_offset = renderer.start_element(tag);

        Self(Internal::new(ElementBuilder {
            tag,
            renderer,
            hash_offset,
            marker: PhantomData,
        }))
    }

    pub fn child(self, child: impl View) -> ElementContent {
        let mut el = match self.0.take_builder() {
            Ok(builder) => builder,
            Err(err) => return ElementContent(Internal::error(err)),
        };

        el.renderer.start_content();
        let fragment = Fragment::new(el.renderer).child(child);
        ElementContent(Internal::new(ElementContentBuilder {
            tag: el.tag,
            hash_offset: el.hash_offset,
            fragment,
        }))
    }

    pub fn any(self) -> AnyView {
        AnyView {
            result: self.render(),
        }
    }

    pub(crate) fn render(self) -> Result<Renderer, crate::Error> {
        let mut builder = self.0.take_builder()?;
        builder.renderer.start_content();
        builder
            .renderer
            .end_element(builder.tag, false, builder.hash_offset);
        Ok(builder.renderer)
    }
}

impl<El> View for Element<El> {
    fn render(self, r: &mut Renderer) -> Result<(), crate::Error> {
        r.append(self.render()?);
        Ok(())
    }
}

impl<El> WithAttribute for Element<El> {
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

impl<El> IntoAnyView for Element<El> {
    fn into_any_view(self) -> AnyView {
        self.any()
    }
}

pub struct ElementContent(Internal<ElementContentBuilder>);

struct ElementContentBuilder {
    tag: &'static str,
    hash_offset: Option<usize>,
    fragment: Fragment,
}

impl ElementContent {
    pub fn child(mut self, child: impl View) -> Self {
        let Some(builder) = self.0.builder_mut() else {
            return self;
        };

        builder.fragment.append_child(child);

        self
    }

    pub fn any(self) -> AnyView {
        AnyView {
            result: self.render(),
        }
    }

    fn render(self) -> Result<Renderer, crate::Error> {
        let builder = self.0.take_builder()?;
        let mut r = builder.fragment.render()?;
        r.end_element(builder.tag, false, builder.hash_offset);
        Ok(r)
    }
}

impl View for ElementContent {
    fn render(self, r: &mut Renderer) -> Result<(), crate::Error> {
        r.append(self.render()?);
        Ok(())
    }
}

impl IntoAnyView for ElementContent {
    fn into_any_view(self) -> AnyView {
        self.any()
    }
}
