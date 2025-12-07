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

impl Context {
    /// The `style` element allows authors to embed CSS style sheets in their documents. The `style`
    /// element is one of several inputs to the styling processing model. The element does not
    /// represent content for the user.
    pub fn style(&self) -> StyleElement {
        StyleElement(Element::new(self.acquire_renderer(), "style"))
    }
}

pub struct StyleElement(Element<marker::Style>);
pub struct StyleContent(ElementContent);

mod marker {
    pub struct Style;
}

impl StyleElement {
    pub fn child<'s>(self, child: impl Into<Cow<'s, str>>) -> StyleContent {
        StyleContent(self.0.child(ScriptEscape(child.into())))
    }
}

impl StyleContent {
    pub fn child<'s>(self, child: impl Into<Cow<'s, str>>) -> Self {
        Self(self.0.child(ScriptEscape(child.into())))
    }
}

impl View for StyleElement {
    fn render(self, r: &mut Renderer) -> Result<(), crate::Error> {
        View::render(self.0, r)
    }
}

impl View for StyleContent {
    fn render(self, r: &mut Renderer) -> Result<(), crate::Error> {
        self.0.render(r)
    }
}

impl WithAttribute for StyleElement {
    fn with_attribute(self, attr: impl Attribute) -> Self {
        Self(self.0.with_attribute(attr))
    }
}

impl Style for StyleElement {}
impl Common for StyleElement {}
impl Global for StyleElement {}
impl Aria for StyleElement {}

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
