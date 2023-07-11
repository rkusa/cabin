pub mod attributes;
pub mod elements;
pub mod events;
pub mod list;
mod raw;

#[doc(inline)]
pub use exports::*;
pub use raw::{raw, Raw};

use self::attributes::Attributes;
use self::elements::{Element, ElementExt};
use crate::render::Renderer;
use crate::view::{RenderFuture, View};

mod exports {
    #[doc(inline)]
    pub use super::elements::anchor::a;
    #[doc(inline)]
    pub use super::elements::body::body;
    #[doc(inline)]
    pub use super::elements::button::button;
    #[doc(inline)]
    pub use super::elements::dialog::dialog;
    #[doc(inline)]
    pub use super::elements::div::div;
    #[doc(inline)]
    pub use super::elements::fieldset::fieldset;
    #[doc(inline)]
    pub use super::elements::form::form;
    #[doc(inline)]
    pub use super::elements::head::head;
    #[doc(inline)]
    pub use super::elements::html::html;
    #[doc(inline)]
    pub use super::elements::input::input;
    #[doc(inline)]
    pub use super::elements::label::label;
    #[doc(inline)]
    pub use super::elements::li::li;
    #[doc(inline)]
    pub use super::elements::link::link;
    #[doc(inline)]
    pub use super::elements::nav::nav;
    #[doc(inline)]
    pub use super::elements::script::script;
    #[doc(inline)]
    pub use super::elements::span::span;
    #[doc(inline)]
    pub use super::elements::time::time;
    #[doc(inline)]
    pub use super::elements::ul::ul;
    #[doc(inline)]
    pub use crate::view::text::{text, Text};

    pub fn doctype() -> impl crate::View {
        super::raw("<!DOCTYPE html>")
    }
}

pub struct Html<V, El, Ext> {
    attributes: Attributes<El, Ext>,
    content: V,
}

impl<V, El, Ext> Html<V, El, Ext>
where
    V: View,
    El: Element,
    Ext: ElementExt,
{
    pub fn new(attributes: impl Into<Attributes<El, Ext>>, content: V) -> Html<V, El, Ext> {
        Html {
            attributes: attributes.into(),
            content,
        }
    }
}

impl<V, El, Ext> View for Html<V, El, Ext>
where
    V: View + 'static,
    El: Element + 'static,
    Ext: ElementExt + 'static,
{
    fn render(self, r: Renderer, include_hash: bool) -> RenderFuture {
        RenderFuture::Future(Box::pin(async move {
            let Html {
                attributes,
                content,
            } = self;

            let mut el = r.element(El::TAG, include_hash)?;
            attributes.render(&mut el)?;

            if !El::is_void_element() {
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
