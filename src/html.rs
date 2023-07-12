pub mod attributes;
pub mod elements;
pub mod events;
pub mod list;
mod raw;

#[doc(inline)]
pub use exports::*;
pub use raw::{raw, Raw};

use self::attributes::Attributes;
use crate::render::Renderer;
use crate::view::{RenderFuture, View};

pub(crate) mod exports {
    #[doc(inline)]
    pub use super::elements::anchor::{self as a, a, Anchor};
    // #[doc(inline)]
    // pub use super::elements::aria::Aria;
    // #[doc(inline)]
    // pub use super::elements::body::{body, Body};
    // #[doc(inline)]
    // pub use super::elements::button::{button, Button};
    // #[doc(inline)]
    // pub use super::elements::common::Common;
    // #[doc(inline)]
    // pub use super::elements::dialog::{dialog, Dialog};
    // #[doc(inline)]
    // pub use super::elements::div::{div, Div};
    // #[doc(inline)]
    // pub use super::elements::fieldset::{fieldset, Fieldset};
    // #[doc(inline)]
    // pub use super::elements::form::{form, Form, FormExt};
    // #[doc(inline)]
    // pub use super::elements::global::Global;
    // #[doc(inline)]
    // pub use super::elements::h1::{h1, H1};
    // #[doc(inline)]
    // pub use super::elements::h2::{h2, H2};
    // #[doc(inline)]
    // pub use super::elements::h3::{h3, H3};
    // #[doc(inline)]
    // pub use super::elements::h4::{h4, H4};
    // #[doc(inline)]
    // pub use super::elements::h5::{h5, H5};
    // #[doc(inline)]
    // pub use super::elements::h6::{h6, H6};
    // #[doc(inline)]
    // pub use super::elements::head::{head, Head};
    // #[doc(inline)]
    // pub use super::elements::html::{html, Html};
    // #[doc(inline)]
    // pub use super::elements::input::{input, Input, InputExt};
    // #[doc(inline)]
    // pub use super::elements::label::{label, Label};
    // #[doc(inline)]
    // pub use super::elements::li::{li, Li};
    // #[doc(inline)]
    // pub use super::elements::link::{link, Link, LinkExt};
    // #[doc(inline)]
    // pub use super::elements::nav::{nav, Nav};
    // #[doc(inline)]
    // pub use super::elements::script::{script, Script, ScriptExt};
    // #[doc(inline)]
    // pub use super::elements::span::{span, Span};
    // #[doc(inline)]
    // pub use super::elements::time::{time, Time};
    // #[doc(inline)]
    // pub use super::elements::ul::{ul, Ul};
    // #[doc(inline)]
    // pub use crate::view::text::{text, Text};

    pub fn doctype() -> impl crate::View {
        super::raw("<!DOCTYPE html>")
    }
}

pub struct Html<A, V> {
    tag: &'static str,
    is_void_element: bool,
    attributes: A,
    content: V,
}

impl<A, V> Html<A, V>
where
    A: Attributes,
    V: View,
{
    pub fn new(tag: &'static str, attributes: A, content: V) -> Html<A, V> {
        Html {
            tag,
            is_void_element: false,
            attributes,
            content,
        }
    }

    pub fn into_void_element(mut self) -> Self {
        self.is_void_element = true;
        self
    }
}

impl<A, V> View for Html<A, V>
where
    A: Attributes,
    V: View,
{
    fn render(self, r: Renderer, include_hash: bool) -> RenderFuture {
        RenderFuture::Future(Box::pin(async move {
            let Html {
                tag,
                is_void_element,
                attributes,
                content,
            } = self;

            let mut el = r.element(tag, include_hash)?;
            attributes.render(&mut el)?;

            if !is_void_element {
                el.content(content).await
            } else {
                el.end(true)
            }
        }))
    }

    fn prime(&mut self) {
        self.content.prime();
    }
}
