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
use crate::view::RenderFuture;

impl Context {
    /// A `script` element allows to include dynamic script and data blocks in their documents.
    pub fn script(&self) -> ScriptElement<'_> {
        ScriptElement(Element::new(self, "script"))
    }
}

pub struct ScriptElement<'v>(Element<'v, marker::Script>);
pub struct ScriptContent<'v>(ElementContent<'v>);

mod marker {
    pub struct Script;
}

impl<'v> ScriptElement<'v> {
    pub fn child(self, child: impl Into<Cow<'v, str>>) -> ScriptContent<'v> {
        ScriptContent(self.0.child(ScriptEscape(child.into())))
    }
}

impl<'v> ScriptContent<'v> {
    pub fn child(self, child: impl Into<Cow<'v, str>>) -> Self {
        Self(self.0.child(ScriptEscape(child.into())))
    }
}

impl<'v> View<'v> for ScriptElement<'v> {
    fn render(self, c: &'v Context, r: Renderer) -> RenderFuture<'v> {
        self.0.render(c, r)
    }
}

impl<'v> View<'v> for ScriptContent<'v> {
    fn render(self, c: &'v Context, r: Renderer) -> RenderFuture<'v> {
        self.0.render(c, r)
    }
}

impl<'v> WithAttribute for ScriptElement<'v> {
    fn with_attribute(self, attr: impl Attribute) -> Self {
        Self(self.0.with_attribute(attr))
    }
}

impl<'v> Script for ScriptElement<'v> {}
impl<'v> Common for ScriptElement<'v> {}
impl<'v> Global for ScriptElement<'v> {}
impl<'v> Aria for ScriptElement<'v> {}

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

pub struct ScriptEscape<'v>(pub Cow<'v, str>);

impl<'v> View<'v> for ScriptEscape<'v> {
    fn render(self, _c: &'v Context, r: Renderer) -> RenderFuture<'v> {
        let mut txt = r.text();
        RenderFuture::ready(
            Escape::script(&mut txt)
                .write_str(&self.0)
                .map_err(crate::error::InternalError::from)
                .map_err(crate::error::Error::from)
                .and_then(|_| Ok(txt.end())),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_script_escape() {
        let c = Context::new(false);
        assert_eq!(
            c.script()
                .child("asd</script>")
                .render(&Context::new(false), Renderer::new(false))
                .await
                .unwrap()
                .end()
                .html,
            r#"<script hash="fc952de5">asd<\/script></script>"#
        );
        assert_eq!(
            c.script()
                .child("asd<!--")
                .render(&Context::new(false), Renderer::new(false))
                .await
                .unwrap()
                .end()
                .html,
            r#"<script hash="e35fcf17">asd<\!--</script>"#
        );
        assert_eq!(
            c.script()
                .child(r#"if (1<2) alert("</script>")"#)
                .render(&Context::new(false), Renderer::new(false))
                .await
                .unwrap()
                .end()
                .html,
            r#"<script hash="b1fe71be">if (1<2) alert("<\/script>")</script>"#
        );
    }
}
