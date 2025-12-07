use std::borrow::Cow;
use std::fmt::Write;

use cabin_macros::Attribute;

use super::anchor::ReferrerPolicy;
use super::aria::Aria;
use super::common::Common;
use super::global::Global;
use super::link::{Blocking, CrossOrigin, FetchPriority, Type};
use crate::View;
use crate::attribute::{Attribute, WithAttribute};
use crate::context::Context;
use crate::element::{Element, ElementContent};
use crate::render::{Escape, Renderer};

impl Context {
    /// A `script` element allows to include dynamic script and data blocks in their documents.
    pub fn script(&self) -> ScriptElement {
        ScriptElement(Element::new(self.acquire_renderer(), "script"))
    }
}

pub struct ScriptElement(Element<marker::Script>);
pub struct ScriptContent(ElementContent);

mod marker {
    pub struct Script;
}

impl ScriptElement {
    pub fn new(renderer: Renderer) -> Self {
        Self(Element::new(renderer, "script"))
    }

    pub fn child<'s>(self, child: impl Into<Cow<'s, str>>) -> ScriptContent {
        ScriptContent(self.0.child(ScriptEscape(child.into())))
    }
}

impl ScriptContent {
    pub fn child<'s>(self, child: impl Into<Cow<'s, str>>) -> Self {
        Self(self.0.child(ScriptEscape(child.into())))
    }
}

impl View for ScriptElement {
    fn render(self, r: &mut Renderer) -> Result<(), crate::Error> {
        View::render(self.0, r)
    }
}

impl View for ScriptContent {
    fn render(self, r: &mut Renderer) -> Result<(), crate::Error> {
        self.0.render(r)
    }
}

impl WithAttribute for ScriptElement {
    fn with_attribute(self, attr: impl Attribute) -> Self {
        Self(self.0.with_attribute(attr))
    }
}

impl Script for ScriptElement {}
impl Common for ScriptElement {}
impl Global for ScriptElement {}
impl Aria for ScriptElement {}

/// A `script` element allows to include dynamic script and data blocks in their documents.
pub trait Script: WithAttribute {
    /// Address of the resource.
    fn src(self, src: impl Into<Cow<'static, str>>) -> Self {
        self.with_attribute(Src(src.into()))
    }

    /// The type of the script.
    fn r#type(self, r#type: impl Into<Cow<'static, str>>) -> Self {
        self.with_attribute(Type(r#type.into()))
    }

    /// Whether to prevent execution in user agents that support module scripts.
    fn no_module(self) -> Self {
        self.with_no_module(true)
    }

    /// Whether to prevent execution in user agents that support module scripts.
    fn with_no_module(self, no_module: bool) -> Self {
        self.with_attribute(NoModule(no_module))
    }

    /// Execute script when available, without blocking while fetching.
    fn r#async(self) -> Self {
        self.with_async(true)
    }

    /// Execute script when available, without blocking while fetching.
    fn with_async(self, r#async: bool) -> Self {
        self.with_attribute(Async(r#async))
    }

    /// Defer script execution.
    fn defer(self) -> Self {
        self.with_defer(true)
    }

    /// Defer script execution.
    fn with_defer(self, defer: bool) -> Self {
        self.with_attribute(Defer(defer))
    }

    /// Handling of crossorigin requests.
    fn cross_origin(self, cross_origin: CrossOrigin) -> Self {
        self.with_attribute(cross_origin)
    }

    /// Integrity metadata used in _Subresource Integrity_ checks.
    fn integrity(self, integrity: impl Into<Cow<'static, str>>) -> Self {
        self.with_attribute(Integrity(integrity.into()))
    }

    /// How much referrer information to send.
    fn referrer_policy(self, referrer_policy: ReferrerPolicy) -> Self {
        self.with_attribute(referrer_policy)
    }

    fn blocking(self) -> Self {
        self.with_blocking(true)
    }

    fn with_blocking(self, blocking: bool) -> Self {
        self.with_attribute(Blocking(blocking))
    }

    /// Sets the priority for fetches initiated by the element.
    fn fetch_priority(self, fetch_priority: FetchPriority) -> Self {
        self.with_attribute(fetch_priority)
    }
}

/// Address of the resource.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Attribute)]
pub struct Src(pub Cow<'static, str>);

/// Whether to prevent execution in user agents that support module scripts.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Attribute)]
pub struct NoModule(pub bool);

/// Execute script when available, without blocking while fetching.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Attribute)]
pub struct Async(pub bool);

/// Defer script execution.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Attribute)]
pub struct Defer(pub bool);

/// Integrity metadata used in _Subresource Integrity_ checks.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Attribute)]
pub struct Integrity(pub Cow<'static, str>);

pub struct ScriptEscape<'s>(pub Cow<'s, str>);

impl<'s> View for ScriptEscape<'s> {
    fn render(self, r: &mut Renderer) -> Result<(), crate::Error> {
        let mut txt = r.text();
        Escape::script(&mut txt)
            .write_str(&self.0)
            .map_err(crate::error::InternalError::from)?;
        txt.end();
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_script_escape() {
        let c = Context::new(false);

        let mut r = c.acquire_renderer();
        c.script().child("asd</script>").render(&mut r).unwrap();
        assert_eq!(
            r.end().html,
            r#"<script hash="54e06b9d">asd<\/script></script>"#
        );

        let mut r = c.acquire_renderer();
        c.script().child("asd<!--").render(&mut r).unwrap();
        assert_eq!(r.end().html, r#"<script hash="7eef666f">asd<\!--</script>"#);

        let mut r = c.acquire_renderer();
        c.script()
            .child(r#"if (1<2) alert("</script>")"#)
            .render(&mut r)
            .unwrap();
        assert_eq!(
            r.end().html,
            r#"<script hash="75755d90">if (1<2) alert("<\/script>")</script>"#
        );
    }
}
