use std::borrow::Cow;

use super::aria::Aria;
use super::common::Common;
use super::global::Global;
use super::link::Blocking;
use super::meta::Media;
use crate::View;
use crate::attribute::{Attribute, WithAttribute};
use crate::context::Context;
use crate::element::{Element, ElementContent};
use crate::html::elements::script::ScriptEscape;
use crate::render::Renderer;
use crate::view::RenderFuture;

impl Context {
    /// The `style` element allows authors to embed CSS style sheets in their documents. The `style`
    /// element is one of several inputs to the styling processing model. The element does not
    /// represent content for the user.
    pub fn style(&self) -> StyleElement<'_> {
        StyleElement(Element::new(self, "style"))
    }
}

pub struct StyleElement<'v>(Element<'v, marker::Style>);
pub struct StyleContent<'v>(ElementContent<'v>);

mod marker {
    pub struct Style;
}

impl<'v> StyleElement<'v> {
    pub fn child(self, child: impl Into<Cow<'v, str>>) -> StyleContent<'v> {
        StyleContent(self.0.child(ScriptEscape(child.into())))
    }
}

impl<'v> StyleContent<'v> {
    pub fn child(self, child: impl Into<Cow<'v, str>>) -> Self {
        Self(self.0.child(ScriptEscape(child.into())))
    }
}

impl<'v> View<'v> for StyleElement<'v> {
    fn render(self, r: Renderer) -> RenderFuture<'v> {
        self.0.render(r)
    }
}

impl<'v> View<'v> for StyleContent<'v> {
    fn render(self, r: Renderer) -> RenderFuture<'v> {
        self.0.render(r)
    }
}

impl<'v> WithAttribute for StyleElement<'v> {
    fn with_attribute(self, attr: impl Attribute) -> Self {
        Self(self.0.with_attribute(attr))
    }
}

impl<'v> Style for StyleElement<'v> {}
impl<'v> Common for StyleElement<'v> {}
impl<'v> Global for StyleElement<'v> {}
impl<'v> Aria for StyleElement<'v> {}

/// The `style` element allows authors to embed CSS style sheets in their documents. The `style`
/// element is one of several inputs to the styling processing model. The element does not represent
/// content for the user.
pub trait Style: WithAttribute {
    /// Applicable media.
    fn media(self, media: impl Into<Cow<'static, str>>) -> Self {
        self.with_attribute(Media(media.into()))
    }

    /// Indicate that the element is potentially render blocking.
    fn blocking(self) -> Self {
        self.with_blocking(true)
    }

    /// Indicate that the element is potentially render blocking.
    fn with_blocking(self, blocking: bool) -> Self {
        self.with_attribute(Blocking(blocking))
    }
}
