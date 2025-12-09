use std::marker::PhantomData;

use crate::attribute::{Attribute, WithAttribute};
use crate::fragment::Fragment;
use crate::render::Renderer;
use crate::view::AnyView;
use crate::view::any::IntoAnyView;
use crate::view::internal::Internal;
use crate::{Context, View};

pub struct Element<El, P = ()>(Internal<ElementBuilder<El, P>>);

struct ElementBuilder<El, P = ()> {
    tag: &'static str,
    renderer: Renderer,
    hash_offset: Option<usize>,
    container: Option<P>,
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
            container: None,
            marker: PhantomData,
        }))
    }

    pub fn with_container<P: Container>(self, container: P) -> Element<El, P> {
        let builder = match self.0.take_builder() {
            Ok(builder) => builder,
            Err(err) => return Element(Internal::error(err)),
        };
        Element(Internal::new(ElementBuilder {
            tag: builder.tag,
            renderer: builder.renderer,
            hash_offset: builder.hash_offset,
            container: Some(container),
            marker: builder.marker,
        }))
    }
}

impl<El, P: Container> Element<El, P> {
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

        if let Some(container) = builder.container {
            let mut r = Context::acquire_renderer_from_task();
            container
                .wrap(AnyView {
                    result: Ok(builder.renderer),
                })
                .render(&mut r)?;
            Ok(r)
        } else {
            Ok(builder.renderer)
        }
    }
}

impl<El, P: Container> View for Element<El, P> {
    fn render(self, r: &mut Renderer) -> Result<(), crate::Error> {
        r.append(self.render()?);
        Ok(())
    }
}

impl<El, P> WithAttribute for Element<El, P> {
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

pub struct ElementContent<El, P = ()>(Internal<ElementContentBuilder<El, P>>);

struct ElementContentBuilder<El, P> {
    tag: &'static str,
    hash_offset: Option<usize>,
    fragment: Fragment,
    container: Option<P>,
    marker: PhantomData<El>,
}

impl<El, P: Container> ElementContent<El, P> {
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

        if let Some(container) = builder.container {
            let mut c = Context::acquire_renderer_from_task();
            container.wrap(AnyView { result: Ok(r) }).render(&mut c)?;
            Ok(c)
        } else {
            Ok(r)
        }
    }
}

impl<El, P: Container> View for ElementContent<El, P> {
    fn render(self, r: &mut Renderer) -> Result<(), crate::Error> {
        r.append(self.render()?);
        Ok(())
    }
}

impl<El, P: Container> IntoAnyView for ElementContent<El, P> {
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

pub trait ElementProxy<El, P = ()>: View + WithAttribute {
    fn into_element(self) -> Element<El, P>;

    fn child<'c>(self, child: impl IntoChild<'c, El> + 'c) -> ElementContent<El, P>
    where
        P: Container,
    {
        self.into_element().child(child)
    }

    fn any(self) -> AnyView
    where
        P: Container,
    {
        self.into_element().any()
    }
}

impl<El, P: Container> ElementProxy<El, P> for Element<El, P> {
    fn into_element(self) -> Element<El, P> {
        self
    }

    fn child<'c>(self, child: impl IntoChild<'c, El>) -> ElementContent<El, P> {
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
            container: el.container,
            marker: PhantomData,
        }))
    }
}

pub trait Container {
    fn wrap(self, content: impl View) -> impl View;
}

impl Container for () {
    fn wrap(self, content: impl View) -> impl View {
        content
    }
}
