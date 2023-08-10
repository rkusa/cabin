use crate::error::InternalError;

pub mod anchor;
pub mod area;
pub mod aria;
pub mod audio;
pub mod base;
pub mod blockquote;
pub mod button;
pub mod canvas;
pub mod col;
pub mod colgroup;
pub mod common;
pub mod data;
pub mod del;
pub mod dialog;
pub mod fieldset;
pub mod form;
pub mod global;
pub mod head;
pub mod html;
pub mod input;
pub mod label;
pub mod li;
pub mod link;
pub mod nav;
pub mod optgroup;
pub mod option;
pub mod script;
pub mod select;
pub mod span;
pub mod textarea;
pub mod time;
pub mod ul;

pub(crate) type SerializeEventFn = dyn FnOnce() -> Result<(u32, String), InternalError>;

macro_rules! vanilla_element {
    ($method_name:ident, $marker_name:ident, $doc:literal) => {
        pub mod $method_name {
            #[allow(unused)]
            use crate::prelude::*;

            #[doc = $doc]
            pub fn $method_name(
                content: impl $crate::View,
            ) -> $crate::html::Html<marker::$marker_name, (), impl $crate::View> {
                #[cfg(debug_assertions)]
                let content = content.boxed();
                $crate::html::Html::new(stringify!($method_name), (), content)
            }

            pub mod marker {
                pub struct $marker_name;
            }

            impl<A: $crate::html::attributes::Attributes, V: 'static>
                $crate::html::elements::common::Common
                for $crate::html::Html<marker::$marker_name, A, V>
            {
            }
            impl<A: $crate::html::attributes::Attributes, V: 'static>
                $crate::html::elements::global::Global
                for $crate::html::Html<marker::$marker_name, A, V>
            {
            }
            impl<A: $crate::html::attributes::Attributes, V: 'static>
                $crate::html::elements::aria::Aria
                for $crate::html::Html<marker::$marker_name, A, V>
            {
            }
        }
    };
}

macro_rules! vanilla_void_element {
    ($method_name:ident, $marker_name:ident, $doc:literal) => {
        pub mod $method_name {
            #[allow(unused)]
            use crate::prelude::*;

            #[doc = $doc]
            pub fn $method_name() -> $crate::html::Html<marker::$marker_name, (), ()> {
                $crate::html::Html::new(stringify!($method_name), (), ())
            }

            pub mod marker {
                pub struct $marker_name;
            }

            impl<A: $crate::html::attributes::Attributes, V: 'static>
                $crate::html::elements::common::Common
                for $crate::html::Html<marker::$marker_name, A, V>
            {
            }
            impl<A: $crate::html::attributes::Attributes, V: 'static>
                $crate::html::elements::global::Global
                for $crate::html::Html<marker::$marker_name, A, V>
            {
            }
            impl<A: $crate::html::attributes::Attributes, V: 'static>
                $crate::html::elements::aria::Aria
                for $crate::html::Html<marker::$marker_name, A, V>
            {
            }
        }
    };
}

vanilla_element!(
    abbr,
    Abbr,
    "The `abbr` element represents an abbreviation or acronym, optionally with its expansion. The \
    [Global::title] attribute may be used to provide an expansion of the abbreviation. The \
    attribute, if specified, must contain an expansion of the abbreviation, and nothing else."
);
vanilla_element!(
    address,
    Address,
    "The address element represents the contact information for its nearest [article] or [body] \
    element ancestor. If that is the [body] element, then the contact information applies to the \
    document as a whole."
);
vanilla_element!(
    article,
    Article,
    "The `article` element represents a complete, or self-contained, composition in a document, \
    page, application, or site and that is, in principle, independently distributable or reusable, \
    e.g. in syndication. This could be a forum post, a magazine or newspaper article, a blog \
    entry, a user-submitted comment, an interactive widget or gadget, or any other independent \
    item of content."
);
vanilla_element!(
    aside,
    Aside,
    "The `aside` element represents a section of a page that consists of content that is \
    tangentially related to the content around the `aside` element, and which could be considered \
    separate from that content. Such sections are often represented as sidebars in printed \
    typography."
);
vanilla_element!(
    b,
    B,
    "The `b` element represents a span of text to which attention is being drawn for utilitarian \
    purposes without conveying any extra importance and with no implication of an alternate voice \
    or mood, such as key words in a document abstract, product names in a review, actionable words \
    in interactive text-driven software, or an article lede."
);
vanilla_element!(
    bdi,
    Bdi,
    "The `bdi` element represents a span of text that is to be isolated from its surroundings for \
    the purposes of bidirectional text formatting.\n\
    The [Global::dir] attribute defaults to [Dir::Auto] on this element (it never inherits from \
    the parent element like with other elements)."
);
vanilla_element!(
    bdo,
    Bdo,
    "The `bdo` element represents explicit text directionality formatting control for its \
    children. It allows authors to override the Unicode bidirectional algorithm by explicitly \
    specifying a direction override.\n\
    Authors must specify the [Global::dir] attribute on this element, with the value [Dir::Ltr] to \
    specify a left-to-right override and with the value [Dir::Rtl] to specify a right-to-left \
    override. The [Dir::Auto] value must not be specified."
);
// Potential events to implement: onafterprint, onbeforeprint, onbeforeunload, onhashchange,
// onlanguagechange, onmessage, onmessageerror, onoffline, ononline, onpagehide, onpageshow,
// onpopstate, onrejectionhandled, onstorage, onunhandledrejection, onunload
vanilla_element!(
    body,
    Body,
    "The `body` element represents the body of an HTML document."
);
vanilla_void_element!(br, Br, "The `br` element represents a line break.");
vanilla_element!(
    caption,
    Caption,
    "The `caption` element represents the title of the [Table] that is its parent, if it has a
    parent and that is a [Table] element."
);
vanilla_element!(
    cite,
    Cite,
    "The `cite` element represents the title of a work"
);
vanilla_element!(
    code,
    Code,
    "The `code` element represents a fragment of computer code."
);
vanilla_element!(
    datalist,
    Datalist,
    "The `datalist` element represents a set of [Option] elements that represent predefined \
    options for other controls. In the rendering, the `datalist` element represents nothing and \
    it, along with its children, should be hidden."
);
vanilla_element!(
    dd,
    Dd,
    "The `dd` element represents the description, definition, or value, part of a term-description \
    group in a description list ([dl] element)."
);

vanilla_element!(
    div,
    Div,
    "The `div` element represents a generic container for flow content."
);
vanilla_element!(h1, H1, "A `h1` heading.");
vanilla_element!(h2, H2, "A `h2` heading.");
vanilla_element!(h3, H3, "A `h3` heading.");
vanilla_element!(h4, H4, "A `h4` heading.");
vanilla_element!(h5, H5, "A `h5` heading.");
vanilla_element!(h6, H6, "A `h6` heading.");

vanilla_element!(
    legend,
    Leged,
    "The `legend` element represents a caption for the rest of the contents of the legend \
    element's parent [fieldset] element, if any."
);

vanilla_element!(
    pre,
    Pre,
    "The pre element represents a block of preformatted text, in which structure is represented \
    by typographic conventions rather than by elements."
);
