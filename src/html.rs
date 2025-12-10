pub mod attributes;
pub mod elements;
pub mod events;
pub mod list;
mod raw;

use std::marker::PhantomData;

#[doc(inline)]
pub use elements::aria::Aria;
#[doc(inline)]
pub use elements::common::Common;
#[doc(inline)]
pub use elements::global::Global;
#[doc(inline)]
pub use h::*;
pub use raw::{Raw, raw};

use self::attributes::{Attributes, WithAttribute};
use crate::pair::Pair;
use crate::render::Renderer;
use crate::view::{AnyView, RenderFuture, View};

pub mod h {
    #[doc(inline)]
    pub use super::elements::abbr::abbr;
    #[doc(inline)]
    pub use super::elements::address::address;
    #[doc(inline)]
    pub use super::elements::anchor::{self as a, Anchor, a};
    #[doc(inline)]
    pub use super::elements::area::{self as area, Area, area};
    #[doc(inline)]
    pub use super::elements::article::article;
    #[doc(inline)]
    pub use super::elements::aside::aside;
    #[doc(inline)]
    pub use super::elements::audio::{self as audio, Audio, audio};
    #[doc(inline)]
    pub use super::elements::b::b;
    #[doc(inline)]
    pub use super::elements::base::{self as base, Base, base};
    #[doc(inline)]
    pub use super::elements::bdi::bdi;
    #[doc(inline)]
    pub use super::elements::bdo::bdo;
    #[doc(inline)]
    pub use super::elements::blockquote::{self as blockquote, Blockquote, blockquote};
    #[doc(inline)]
    pub use super::elements::body::body;
    #[doc(inline)]
    pub use super::elements::br::br;
    #[doc(inline)]
    pub use super::elements::button::{self as button, Button, button};
    #[doc(inline)]
    pub use super::elements::canvas::{self as canvas, Canvas, canvas};
    #[doc(inline)]
    pub use super::elements::caption::caption;
    #[doc(inline)]
    pub use super::elements::cite::cite;
    #[doc(inline)]
    pub use super::elements::code::code;
    #[doc(inline)]
    pub use super::elements::col::{self as col, Col, col};
    #[doc(inline)]
    pub use super::elements::colgroup::{self as colgroup, Colgroup, colgroup};
    #[doc(inline)]
    pub use super::elements::data::{self as data, Data, data};
    #[doc(inline)]
    pub use super::elements::datalist::datalist;
    #[doc(inline)]
    pub use super::elements::dd::dd;
    #[doc(inline)]
    pub use super::elements::del::{self as del, Del, del};
    #[doc(inline)]
    pub use super::elements::details::{self as details, Details, details};
    #[doc(inline)]
    pub use super::elements::dfn::dfn;
    #[doc(inline)]
    pub use super::elements::dialog::{self as dialog, Dialog, dialog};
    #[doc(inline)]
    pub use super::elements::div::div;
    #[doc(inline)]
    pub use super::elements::dl::dl;
    #[doc(inline)]
    pub use super::elements::dt::dt;
    #[doc(inline)]
    pub use super::elements::em::em;
    #[doc(inline)]
    pub use super::elements::embed::{self as embed, Embed, embed};
    #[doc(inline)]
    pub use super::elements::fieldset::{self as fieldset, Fieldset, fieldset};
    #[doc(inline)]
    pub use super::elements::figcaption::figcaption;
    #[doc(inline)]
    pub use super::elements::figure::figure;
    #[doc(inline)]
    pub use super::elements::footer::footer;
    #[doc(inline)]
    pub use super::elements::form::{self as form, Form, form};
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
    pub use super::elements::head::head;
    #[doc(inline)]
    pub use super::elements::header::header;
    #[doc(inline)]
    pub use super::elements::hgroup::hgroup;
    #[doc(inline)]
    pub use super::elements::hr::hr;
    #[doc(inline)]
    pub use super::elements::html::html;
    #[doc(inline)]
    pub use super::elements::i::i;
    #[doc(inline)]
    pub use super::elements::iframe::{self as iframe, IFrame, iframe};
    #[doc(inline)]
    pub use super::elements::img::{self as img, Img, img};
    #[doc(inline)]
    pub use super::elements::input::{self as input, Input, input};
    #[doc(inline)]
    pub use super::elements::ins::{self as ins, Ins, ins};
    #[doc(inline)]
    pub use super::elements::kbd::kbd;
    #[doc(inline)]
    pub use super::elements::label::{self as label, Label, label};
    #[doc(inline)]
    pub use super::elements::legend::legend;
    #[doc(inline)]
    pub use super::elements::li::li;
    #[doc(inline)]
    pub use super::elements::link::{self as link, Link, link};
    #[doc(inline)]
    pub use super::elements::main::main;
    #[doc(inline)]
    pub use super::elements::map::{self as map, Map, map};
    #[doc(inline)]
    pub use super::elements::mark::mark;
    #[doc(inline)]
    pub use super::elements::menu::menu;
    #[doc(inline)]
    pub use super::elements::meta::{self as meta, Meta, meta};
    #[doc(inline)]
    pub use super::elements::meter::{self as meter, Meter, meter};
    #[doc(inline)]
    pub use super::elements::nav::nav;
    #[doc(inline)]
    pub use super::elements::noscript::noscript;
    #[doc(inline)]
    pub use super::elements::object::{self as object, Object, object};
    #[doc(inline)]
    pub use super::elements::ol::{self as ol, Ol, ol};
    #[doc(inline)]
    pub use super::elements::optgroup::{self as optgroup, OptGroup, optgroup};
    #[doc(inline)]
    pub use super::elements::option::{self as option, SelectOption, option};
    #[doc(inline)]
    pub use super::elements::output::{self as output, Output, output};
    #[doc(inline)]
    pub use super::elements::p::p;
    #[doc(inline)]
    pub use super::elements::picture::picture;
    #[doc(inline)]
    pub use super::elements::pre::pre;
    #[doc(inline)]
    pub use super::elements::progress::{self as progress, Progress, progress};
    #[doc(inline)]
    pub use super::elements::q::{self as q, Q, q};
    #[doc(inline)]
    pub use super::elements::rp::rp;
    #[doc(inline)]
    pub use super::elements::rt::rt;
    #[doc(inline)]
    pub use super::elements::ruby::ruby;
    #[doc(inline)]
    pub use super::elements::s::s;
    #[doc(inline)]
    pub use super::elements::samp::samp;
    #[doc(inline)]
    pub use super::elements::script::{self as script, Script, script};
    #[doc(inline)]
    pub use super::elements::search::search;
    #[doc(inline)]
    pub use super::elements::section::section;
    #[doc(inline)]
    pub use super::elements::select::{self as select, Select, select};
    #[doc(inline)]
    pub use super::elements::slot::{self as slot, Slot, slot};
    #[doc(inline)]
    pub use super::elements::small::small;
    #[doc(inline)]
    pub use super::elements::source::{self as source, Source, source};
    #[doc(inline)]
    pub use super::elements::span::span;
    #[doc(inline)]
    pub use super::elements::strong::strong;
    #[doc(inline)]
    pub use super::elements::style::{self as style, Style, style};
    #[doc(inline)]
    pub use super::elements::sub::sub;
    #[doc(inline)]
    pub use super::elements::summary::summary;
    #[doc(inline)]
    pub use super::elements::sup::sup;
    #[doc(inline)]
    pub use super::elements::table::table;
    #[doc(inline)]
    pub use super::elements::tbody::tbody;
    #[doc(inline)]
    pub use super::elements::td::{self as td, Td, td};
    #[doc(inline)]
    pub use super::elements::template::template;
    #[doc(inline)]
    pub use super::elements::textarea::{self as textarea, Textarea, textarea};
    #[doc(inline)]
    pub use super::elements::tfoot::tfoot;
    #[doc(inline)]
    pub use super::elements::th::{self as th, Th, th};
    #[doc(inline)]
    pub use super::elements::thead::thead;
    #[doc(inline)]
    pub use super::elements::time::{self as time, Time, time};
    #[doc(inline)]
    pub use super::elements::title::{title, title_update};
    #[doc(inline)]
    pub use super::elements::tr::tr;
    #[doc(inline)]
    pub use super::elements::track::{self as track, Track, track};
    #[doc(inline)]
    pub use super::elements::u::u;
    #[doc(inline)]
    pub use super::elements::ul::ul;
    #[doc(inline)]
    pub use super::elements::var::var;
    #[doc(inline)]
    pub use super::elements::video::{self as video, Video, video};
    #[doc(inline)]
    pub use super::elements::wbr::wbr;
    pub use crate::html::raw;
    #[doc(inline)]
    pub use crate::view::text::{Text, text};
    pub use crate::view::view;

    pub fn doctype() -> impl crate::View {
        crate::view::UpdateView::hidden_on_update(super::raw("<!DOCTYPE html>"))
    }
}

pub struct Html<El, A> {
    tag: &'static str,
    is_void_element: bool,
    attributes: A,
    pub(crate) content: AnyView,
    marker: PhantomData<El>,
}

impl<El, A> Html<El, A>
where
    A: Attributes,
{
    pub fn new(tag: &'static str, attributes: A, content: impl View) -> Html<El, A> {
        Html {
            tag,
            is_void_element: false,
            attributes,
            content: content.into_any_view(),
            marker: PhantomData,
        }
    }

    pub fn into_void_element(mut self) -> Self {
        self.is_void_element = true;
        self
    }
}

impl<El, A> View for Html<El, A>
where
    El: Send + 'static,
    A: Attributes,
{
    fn render(self, r: Renderer) -> RenderFuture {
        let Html {
            tag,
            is_void_element,
            attributes,
            content,
            marker: _,
        } = self;

        let mut el = r.element(tag);
        if let Err(err) = attributes.render(&mut el) {
            return RenderFuture::Ready(Err(err));
        }

        if !is_void_element {
            el.content(content)
        } else {
            RenderFuture::Ready(Ok(el.end(true)))
        }
    }
}

impl<El, A> WithAttribute for Html<El, A>
where
    A: Attributes,
{
    type Output<T>
        = Html<El, Pair<T, A>>
    where
        T: Attributes;

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
