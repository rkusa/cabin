use std::marker::PhantomData;

use crate::attribute::{Attribute, WithAttribute};
use crate::fragment::Fragment;
use crate::render::Renderer;
use crate::view::AnyView;
use crate::view::any::IntoAnyView;
use crate::view::internal::Internal;
use crate::{Context, View};

pub struct Element<El>(Internal<ElementBuilder<El>>);

struct ElementBuilder<El> {
    tag: &'static str,
    renderer: Renderer,
    hash_offset: Option<usize>,
    marker: PhantomData<El>,
}

impl<El> Element<El> {
    pub fn new(tag: &'static str) -> Self {
        let mut renderer = Context::acquire_renderer_from_task();
        let hash_offset = renderer.start_element(tag);

        Self(Internal::new(ElementBuilder {
            tag,
            renderer,
            hash_offset,
            marker: PhantomData,
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

pub struct ElementContent<El>(Internal<ElementContentBuilder<El>>);

struct ElementContentBuilder<El> {
    tag: &'static str,
    hash_offset: Option<usize>,
    fragment: Fragment,
    marker: PhantomData<El>,
}

impl<El> ElementContent<El> {
    pub fn child<'c>(mut self, child: impl IntoChild<'c, El>) -> Self {
        let Some(builder) = self.0.builder_mut() else {
            return self;
        };

        builder.fragment.append_child(child.into_child());

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

impl<El> View for ElementContent<El> {
    fn render(self, r: &mut Renderer) -> Result<(), crate::Error> {
        r.append(self.render()?);
        Ok(())
    }
}

impl<El> IntoAnyView for ElementContent<El> {
    fn into_any_view(self) -> AnyView {
        self.any()
    }
}

pub trait IntoChild<'v, El> {
    fn into_child(self) -> impl View + 'v;
}

impl<'v, V: crate::View + 'v> crate::element::IntoChild<'v, ()> for V {
    fn into_child(self) -> impl crate::View + 'v {
        self
    }
}

pub trait ElementProxy<El>: View + WithAttribute {
    fn into_element(self) -> Element<El>;

    fn child<'c>(self, child: impl IntoChild<'c, El> + 'c) -> ElementContent<El> {
        self.into_element().child(child)
    }
}

impl<El> ElementProxy<El> for Element<El> {
    fn into_element(self) -> Element<El> {
        self
    }

    fn child<'c>(self, child: impl IntoChild<'c, El>) -> ElementContent<El> {
        let mut el = match self.0.take_builder() {
            Ok(builder) => builder,
            Err(err) => return ElementContent(Internal::error(err)),
        };

        el.renderer.start_content();
        let fragment = Fragment::new(el.renderer).child(child.into_child());
        ElementContent(Internal::new(ElementContentBuilder {
            tag: el.tag,
            hash_offset: el.hash_offset,
            fragment,
            marker: PhantomData,
        }))
    }
}
