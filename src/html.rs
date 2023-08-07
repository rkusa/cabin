pub mod attributes;
pub mod elements;
pub mod events;
pub mod list;
mod raw;

use std::marker::PhantomData;

#[doc(inline)]
pub use exports::*;
pub use raw::{raw, Raw};

use self::attributes::{Attributes, Pair, WithAttribute};
use crate::render::Renderer;
use crate::view::{RenderFuture, View};

pub(crate) mod exports {
    #[doc(inline)]
    pub use super::elements::abbr::abbr;
    #[doc(inline)]
    pub use super::elements::address::address;
    #[doc(inline)]
    pub use super::elements::anchor::{self as a, a, Anchor};
    #[doc(inline)]
    pub use super::elements::area::{self as area, area};
    #[doc(inline)]
    pub use super::elements::aria::*;
    #[doc(inline)]
    pub use super::elements::article::article;
    #[doc(inline)]
    pub use super::elements::aside::aside;
    #[doc(inline)]
    pub use super::elements::b::b;
    #[doc(inline)]
    pub use super::elements::body::body;
    #[doc(inline)]
    pub use super::elements::button::{self as button, button, Button};
    #[doc(inline)]
    pub use super::elements::common::*;
    #[doc(inline)]
    pub use super::elements::dialog::{self as dialog, dialog, Dialog};
    #[doc(inline)]
    pub use super::elements::div::div;
    #[doc(inline)]
    pub use super::elements::fieldset::{self as fieldset, fieldset};
    #[doc(inline)]
    pub use super::elements::form::{self as form, form, Form};
    #[doc(inline)]
    pub use super::elements::global::*;
    #[doc(inline)]
    pub use super::elements::h1::h1;
    #[doc(inline)]
    pub use super::elements::h2::h2;
    #[doc(inline)]
    pub use super::elements::h3::h3;
    #[doc(inline)]
    pub use super::elements::h4::h4;
    #[doc(inline)]
    pub use super::elements::h5::h5;
    #[doc(inline)]
    pub use super::elements::h6::h6;
    #[doc(inline)]
    pub use super::elements::head::{self as head, head};
    #[doc(inline)]
    pub use super::elements::html::{self as html, html};
    #[doc(inline)]
    pub use super::elements::input::{self as input, input, Input};
    #[doc(inline)]
    pub use super::elements::label::{self as label, label, Label};
    #[doc(inline)]
    pub use super::elements::legend::legend;
    #[doc(inline)]
    pub use super::elements::li::{self as li, li};
    #[doc(inline)]
    pub use super::elements::link::{self as link, link, Link};
    #[doc(inline)]
    pub use super::elements::nav::{self as nav, nav};
    #[doc(inline)]
    pub use super::elements::optgroup::{self as optgroup, optgroup, OptGroup};
    #[doc(inline)]
    pub use super::elements::option::{self as option, option, SelectOption};
    #[doc(inline)]
    pub use super::elements::pre::pre;
    #[doc(inline)]
    pub use super::elements::script::{self as script, script, Script};
    #[doc(inline)]
    pub use super::elements::select::{self as select, select, Select};
    #[doc(inline)]
    pub use super::elements::span::{self as span, span};
    #[doc(inline)]
    pub use super::elements::textarea::{self as textarea, textarea, Textarea};
    #[doc(inline)]
    pub use super::elements::time::{self as time, time, Time};
    #[doc(inline)]
    pub use super::elements::ul::{self as ul, ul};
    #[doc(inline)]
    pub use crate::view::text::{text, Text};

    pub fn doctype() -> impl crate::View {
        super::raw("<!DOCTYPE html>")
    }
}

pub struct Html<El, A, V> {
    tag: &'static str,
    is_void_element: bool,
    attributes: A,
    content: V,
    marker: PhantomData<El>,
}

impl<El, A, V> Html<El, A, V>
where
    A: Attributes,
    V: View,
{
    pub fn new(tag: &'static str, attributes: A, content: V) -> Html<El, A, V> {
        Html {
            tag,
            is_void_element: false,
            attributes,
            content,
            marker: PhantomData,
        }
    }

    pub fn into_void_element(mut self) -> Self {
        self.is_void_element = true;
        self
    }
}

impl<El, A, V> View for Html<El, A, V>
where
    El: 'static,
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
                marker: _,
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

impl<El, A, V> WithAttribute for Html<El, A, V>
where
    A: Attributes,
{
    type Output<T> = Html<El, Pair<T, A>, V> where T: Attributes;

    fn with_attribute<T: Attributes>(self, attr: T) -> Self::Output<T> {
        Html {
            tag: self.tag,
            is_void_element: self.is_void_element,
            attributes: self.attributes.with(attr),
            content: self.content,
            marker: PhantomData,
        }
    }

    fn get_attribute<T: 'static>(&self) -> Option<&T> {
        self.attributes.get()
    }

    fn get_attribute_mut<T: 'static>(&mut self) -> Option<&mut T> {
        self.attributes.get_mut()
    }
}
