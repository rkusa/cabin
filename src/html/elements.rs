use crate::error::InternalError;

pub mod anchor;
pub mod area;
pub mod aria;
pub mod audio;
pub mod base;
pub mod blockquote;
pub mod body;
pub mod button;
pub mod canvas;
pub mod col;
pub mod colgroup;
pub mod common;
pub mod data;
pub mod del;
pub mod details;
pub mod dialog;
pub mod embed;
pub mod fieldset;
pub mod form;
pub mod global;
pub mod head;
pub mod html;
pub mod iframe;
pub mod img;
pub mod input;
pub mod ins;
pub mod label;
pub mod link;
pub mod map;
pub mod meta;
pub mod meter;
pub mod object;
pub mod ol;
pub mod optgroup;
pub mod option;
pub mod output;
pub mod progress;
pub mod q;
pub mod script;
pub mod select;
pub mod slot;
pub mod source;
pub mod span;
pub mod style;
pub mod td;
pub mod textarea;
pub mod th;
pub mod time;
pub mod title;
pub mod track;
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
                $crate::html::Html::new(stringify!($method_name), (), ()).into_void_element()
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
    "The address element represents the contact information for its nearest [h::article] or \
    [h::body] element ancestor. If that is the [h::body] element, then the contact information \
    applies to the document as a whole."
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
    The [Global::dir] attribute defaults to [cabin::html::elements::global::Dir::Auto] on this \
    element (it never inherits from the parent element like with other elements)."
);
vanilla_element!(
    bdo,
    Bdo,
    "The `bdo` element represents explicit text directionality formatting control for its \
    children. It allows authors to override the Unicode bidirectional algorithm by explicitly \
    specifying a direction override.\n\
    Authors must specify the [Global::dir] attribute on this element, with the value \
    [cabin::html::elements::global::Dir::Ltr] to specify a left-to-right override and with the \
    value [cabin::html::elements::global::Dir::Rtl] to specify a right-to-left override. The \
    [cabin::html::elements::global::Dir::Auto] value must not be specified."
);
// Potential events to implement: onafterprint, onbeforeprint, onbeforeunload, onhashchange,
// onlanguagechange, onmessage, onmessageerror, onoffline, ononline, onpagehide, onpageshow,
// onpopstate, onrejectionhandled, onstorage, onunhandledrejection, onunload
vanilla_void_element!(br, Br, "The `br` element represents a line break.");
vanilla_element!(
    caption,
    Caption,
    "The `caption` element represents the title of the [h::table] that is its parent, if it has a
    parent and that is a [h::table] element."
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
    "The `datalist` element represents a set of [fn@h::option] elements that represent predefined \
    options for other controls. In the rendering, the `datalist` element represents nothing and \
    it, along with its children, should be hidden."
);
vanilla_element!(
    dd,
    Dd,
    "The `dd` element represents the description, definition, or value, part of a term-description \
    group in a description list ([h::dl] element)."
);
vanilla_element!(
    dfn,
    Dfn,
    "The `dfn` element represents the defining instance of a term. The paragraph, description list \
    group, or section that is the nearest ancestor of the `dfn` element must also contain the \
    definition(s) for the term given by the `dfn` element."
);
vanilla_element!(
    div,
    Div,
    "The `div` element represents a generic container for flow content."
);
vanilla_element!(
    dl,
    Dl,
    "The `dl` element represents an association list consisting of zero or more name-value groups \
    (a description list)."
);
vanilla_element!(
    dt,
    Dt,
    "The `dt` element represents the term, or name, part of a term-description group in a \
    description list ([h::dl] element)."
);
vanilla_element!(
    em,
    Em,
    "The `em` element represents stress emphasis of its contents."
);
vanilla_element!(
    figcaption,
    FigCaption,
    "The `figcaption` element represents a caption or legend for the rest of the contents of the \
    `figcaption` element's parent [h::figure] element, if any."
);
vanilla_element!(
    figure,
    Figure,
    "The `figure` element represents some flow content, optionally with a caption, that is \
    self-contained (like a complete sentence) and is typically referenced as a single unit from \
    the main flow of the document."
);
vanilla_element!(
    footer,
    Footer,
    "The `footer` element represents a footer for its nearest ancestor sectioning content element, \
    or for the body element if there is no such ancestor. A footer typically contains information \
    about its section such as who wrote it, links to related documents, copyright data, and the \
    like."
);
vanilla_element!(h1, H1, "A `h1` heading.");
vanilla_element!(h2, H2, "A `h2` heading.");
vanilla_element!(h3, H3, "A `h3` heading.");
vanilla_element!(h4, H4, "A `h4` heading.");
vanilla_element!(h5, H5, "A `h5` heading.");
vanilla_element!(h6, H6, "A `h6` heading.");
vanilla_element!(
    header,
    Header,
    "The `header` element represents a group of introductory or navigational aids."
);
vanilla_element!(
    hgroup,
    HGroup,
    "The `hgroup` element represents a heading and related content. The element may be used to \
    group an [h::h1]–[h::h6] element with one or more [h::p] elements containing content \
    representing a subheading, alternative title, or tagline."
);
vanilla_void_element!(
    hr,
    Hr,
    "The `hr` element represents a paragraph-level thematic break, e.g., a scene change in a \
    story, or a transition to another topic within a section of a reference book; alternatively, \
    it represents a separator between a set of options of a [fn@h::select] element."
);
vanilla_element!(
    i,
    I,
    "The `i` element represents a span of text in an alternate voice or mood, or otherwise offset \
    from the normal prose in a manner indicating a different quality of text, such as a taxonomic \
    designation, a technical term, an idiomatic phrase from another language, transliteration, a \
    thought, or a ship name in Western texts."
);
vanilla_element!(
    kbd,
    Kbd,
    "The `kbd` element represents user input (typically keyboard input, although it may also be \
    used to represent other input, such as voice commands)."
);
vanilla_element!(
    legend,
    Leged,
    "The `legend` element represents a caption for the rest of the contents of the legend \
    element's parent [fn@h::fieldset] element, if any."
);
vanilla_element!(
    li,
    Li,
    "The `li` element represents a list item. If its parent element is an [fn@h::ol], [fn@h::ul], \
    or [h::menu] element, then the element is an item of the parent element's list, as defined for \
    those elements. Otherwise, the list item has no defined list-related relationship to any other \
    `li` element."
);
vanilla_element!(
    main,
    Main,
    "The `main` element represents the dominant contents of the document."
);
vanilla_element!(
    mark,
    Mark,
    "The `mark` element represents a run of text in one document marked or highlighted for \
    reference purposes, due to its relevance in another context. When used in a quotation or other \
    block of text referred to from the prose, it indicates a highlight that was not originally \
    present but which has been added to bring the reader's attention to a part of the text that \
    might not have been considered important by the original author when the block was originally \
    written, but which is now under previously unexpected scrutiny. When used in the main prose of \
    a document, it indicates a part of the document that has been highlighted due to its likely \
    relevance to the user's current activity."
);
vanilla_element!(
    menu,
    Menu,
    "The `menu` element represents a toolbar consisting of its contents, in the form of an \
    unordered list of items (represented by [h::li] elements), each of which represents a command \
    that the user can perform or activate."
);
vanilla_element!(
    nav,
    Nav,
    "The `nav` element represents a section of a page that links to other pages or to parts within \
    the page: a section with navigation links."
);
vanilla_element!(
    noscript,
    NoScript,
    "The `noscript` element represents nothing if scripting is enabled, and represents its \
    children if scripting is disabled. It is used to present different markup to user agents that \
    support scripting and those that don't support scripting, by affecting how the document is \
    parsed."
);
vanilla_element!(p, P, "The p element represents a paragraph.");
vanilla_element!(
    picture,
    Picture,
    "The `picture` element is a container which provides multiple sources to its contained \
    [fn@h::img] element to allow authors to declaratively control or give hints to the user agent \
    about which image resource to use, based on the screen pixel density, viewport size, image \
    format, and other factors. It represents its children."
);
vanilla_element!(
    pre,
    Pre,
    "The `pre` element represents a block of preformatted text, in which structure is represented \
    by typographic conventions rather than by elements."
);
vanilla_element!(
    rp,
    Rp,
    "The `rp` element can be used to provide parentheses or other content around a [h::ruby] text \
    component of a ruby annotation, to be shown by user agents that don't support ruby annotations."
);
vanilla_element!(
    rt,
    Rt,
    "The `rt` element marks the ruby text component of a ruby annotation. When it is the child of \
    a [h::ruby] element, it doesn't represent anything itself, but the [h::ruby] element uses it \
    as part of determining what it represents."
);
vanilla_element!(
    ruby,
    Ruby,
    "The `ruby` element allows one or more spans of phrasing content to be marked with ruby \
    annotations. Ruby annotations are short runs of text presented alongside base text, primarily \
    used in East Asian typography as a guide for pronunciation or to include other annotations. In \
    Japanese, this form of typography is also known as furigana."
);
vanilla_element!(
    s,
    S,
    "The `s` element represents contents that are no longer accurate or no longer relevant."
);
vanilla_element!(
    samp,
    Samp,
    "The `samp` element represents sample or quoted output from another program or computing \
    system."
);
vanilla_element!(
    search,
    Search,
    "The `search` element represents a part of a document or application that contains a set of \
    form controls or other content related to performing a search or filtering operation. This \
    could be a search of the web site or application; a way of searching or filtering search \
    results on the current web page; or a global or Internet-wide search function."
);
vanilla_element!(
    section,
    Section,
    "The section element represents a generic section of a document or application. A section, in \
    this context, is a thematic grouping of content, typically with a heading."
);
vanilla_element!(
    small,
    Small,
    "The `small` element represents side comments such as small print."
);
vanilla_element!(
    strong,
    Strong,
    "The `strong` element represents strong importance, seriousness, or urgency for its contents."
);
vanilla_element!(sup, Sup, "The `sup` element represents a superscript.");
vanilla_element!(sub, Sub, "The `sub` element represents a subscript.");
vanilla_element!(
    summary,
    Summary,
    "The `summary` element represents a summary, caption, or legend for the rest of the contents \
    of the `summary` element's parent [fn@h::details] element, if any."
);
vanilla_element!(
    table,
    Table,
    "The `table` element represents data with more than one dimension, in the form of a table."
);
vanilla_element!(
    tbody,
    TBody,
    "The `tbody` element represents a block of rows that consist of a body of data for the parent \
    [h::table] element, if the `tbody` element has a parent and it is a table."
);
vanilla_element!(
    template,
    Template,
    "The `template` element is used to declare fragments of HTML that can be cloned and inserted \
    in the document by script."
);
vanilla_element!(
    tfoot,
    TFoot,
    "The `tfoot` element represents the block of rows that consist of the column summaries \
    (footers) for the parent [h::table] element, if the `tfoot` element has a parent and it is a \
    table."
);
vanilla_element!(
    thead,
    THead,
    "The `thead` element represents the block of rows that consist of the column labels (headers) \
    and any ancillary non-header cells for the parent [h::table] element, if the `thead` element \
    has a parent and it is a table."
);
vanilla_element!(
    tr,
    Tr,
    "The `tr` element represents a row of cells in a table."
);
vanilla_element!(
    u,
    U,
    "The `u` element represents a span of text with an unarticulated, though explicitly rendered, \
    non-textual annotation, such as labeling the text as being a proper name in Chinese text (a \
    Chinese proper name mark), or labeling the text as being misspelt."
);
