use std::borrow::Cow;
use std::fmt;

use cabin_macros::Attributes;
use cabin_macros::{Attributes2, Element};

use crate::html::attributes::{Attributes2, Pair};

use crate::html::attributes::Attributes;

#[derive(Default, Attributes)]
pub struct GlobalAttributes {
    /// Used by the user agent as a guide for creating a keyboard shortcut that activates or
    /// focuses the element.
    #[attributes(attribute_name = "accesskey")]
    access_key: Option<Cow<'static, str>>,

    /// Hints the user-agent of how to automatically capitalize input (from non-physical
    // keyboards; e.g. virtual keyboards, voice input).
    #[attributes(attribute_name = "autocapitalize")]
    auto_capitalize: AutoCapitalize,

    /// Indicate that the element is to be focused as soon as the page is loaded.
    #[attributes(attribute_name = "autofocus")]
    auto_focus: bool,

    /// Indicating whether the element should be editable by the user.
    #[attributes(attribute_name = "contenteditable")]
    content_editable: bool,

    /// The element's text directionality.
    dir: Option<Dir>,

    /// Indicate whether the element can be dragged.
    draggable: bool,

    /// Indicate the action label (or icon) to present for the enter key on virtual keyboards.
    #[attributes(attribute_name = "enterkeyhint")]
    enter_key_hint: Option<EnterKeyHint>,

    /// Hide the element (visually and from screen-readers).
    hidden: Option<Hidden>,

    /// Mark the element as not presently accessible (e.g. when overlayed by a loading state).
    inert: bool,

    /// Hint an input mechanism that would be most helpful for users entering content.
    #[attributes(attribute_name = "inputmode")]
    input_mode: Option<InputMode>,

    /// The element should behave like the defined custom element.
    is: Option<Cow<'static, str>>,

    /// Unique, global identifier of the item (`item_scope` and `item_type` must also be defined).
    #[attributes(attribute_name = "itemid")]
    item_id: Option<Cow<'static, str>>,

    /// A microdata name-value pair (this is the name – either a string or URL; the element's
    /// content is the value).
    #[attributes(attribute_name = "itemprop")]
    item_prop: Option<Cow<'static, str>>,

    /// List of element IDs elsewehre in the document with additional properties.
    #[attributes(attribute_name = "itemref")]
    item_ref: Option<Cow<'static, str>>,

    /// Create a new item by scoping the the descendent properties together (`item_type` must also
    /// be defined).
    #[attributes(attribute_name = "itemscope")]
    item_scope: bool,

    /// URL of the vocabulary that will be used to define item properties in the data structure.
    #[attributes(attribute_name = "itemtype")]
    item_type: Option<Cow<'static, str>>,

    /// Primary language of the element's contents.
    lang: Option<Cow<'static, str>>,

    /// Cryptographic nonce ("number used once") which can be used by Content Security Policy to
    /// determine whether or not a given fetch will be allowed to proceed.
    nonce: Option<Cow<'static, str>>,

    /// Don't render the element until it becomes shown, at which point it will be rendered on top
    /// of other page content.
    popover: bool,

    /// The slot name this element is assigned to.
    slot: Option<Cow<'static, str>>,

    /// Explicitly enable or disable spelling and grammar checking for the element's contents.
    spellcheck: Option<bool>,

    /// Inline CSS.
    style: Option<Cow<'static, str>>,

    /// Specifify how and in which order the element can be focused.
    #[attributes(attribute_name = "tabindex")]
    tab_index: Option<TabIndex>,

    /// Advisory information for the element, such as would be appropriate for a tooltip.
    title: Option<Cow<'static, str>>,

    #[attributes(skip)]
    translate: Option<DontTranslate>,
}

impl<El> Attributes<El> {
    /// Indicate that the contents of this element should not be translated when the page is
    /// localized.
    pub fn no_translate(mut self) -> Self {
        AsMut::<GlobalAttributes>::as_mut(&mut self).translate = Some(DontTranslate);
        self
    }
}

/// Autocapitalization hints.
#[derive(Default, Hash, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum AutoCapitalize {
    /// No autocapitalization.
    #[default]
    None,

    /// The first letter of each sentence defaults to a capital letter.
    Sentences,

    /// The first letter of each word defaults to a capital letter.
    Words,

    /// All letters should default to uppercase.
    Characters,
}

impl fmt::Display for AutoCapitalize {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AutoCapitalize::None => f.write_str("none"),
            AutoCapitalize::Sentences => f.write_str("sentences"),
            AutoCapitalize::Words => f.write_str("words"),
            AutoCapitalize::Characters => f.write_str("characters"),
        }
    }
}

/// Text directionality.
#[derive(Hash, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum Dir {
    /// Directionally isolated left-to-right text.
    Ltr,

    /// Directionally isolated right-to-left text.
    Rtl,

    /// Directionally isolated text, but direction is to be determined by user-agent.
    Auto,
}

impl fmt::Display for Dir {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Dir::Ltr => f.write_str("ltr"),
            Dir::Rtl => f.write_str("rtl"),
            Dir::Auto => f.write_str("auto"),
        }
    }
}

/// The action label (or icon) to present for the enter key on virtual keyboards
#[derive(Hash, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum EnterKeyHint {
    /// Insert new line.
    Enter,

    /// Nothing more to input – close input method editor.
    Done,

    /// Take user to target of the typed text.
    Go,

    /// Take user to next field that will accept text.
    Next,

    /// Take user to previous field that will accept text.
    Previous,

    /// Take user to search result for typed text.
    Search,

    /// Deliver the text to its target.
    Send,
}

impl fmt::Display for EnterKeyHint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EnterKeyHint::Enter => f.write_str("enter"),
            EnterKeyHint::Done => f.write_str("done"),
            EnterKeyHint::Go => f.write_str("go"),
            EnterKeyHint::Next => f.write_str("next"),
            EnterKeyHint::Previous => f.write_str("previous"),
            EnterKeyHint::Search => f.write_str("search"),
            EnterKeyHint::Send => f.write_str("send"),
        }
    }
}

/// The action label (or icon) to present for the enter key on virtual keyboards
#[derive(Hash, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum Hidden {
    /// Do not render element.
    Hidden,

    /// Do not render, unless found by searching the page or via fragment navigation.
    UntilFound,
}

impl fmt::Display for Hidden {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Hidden::Hidden => f.write_str("hidden"),
            Hidden::UntilFound => f.write_str("until-found"),
        }
    }
}

/// The input mechanism that would be most helpful for users entering content.
#[derive(Hash, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum InputMode {
    /// Do not display a virtual keyboard (useful for when rendering your own keyboard).
    None,

    /// Text input in user's locale.
    Text,

    /// Telephone number input.
    Tel,

    /// Text input in user's locale, with keys for aiding in the input of URLs.
    Url,

    /// Text input in user's locale, with keys for aiding in the input of email addresses.
    Email,

    /// Numeric input.
    Numeric,

    /// Fractional numeric input.
    Decimal,

    /// Text input in user's locale, optimized for search.
    Search,
}

impl fmt::Display for InputMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            InputMode::None => f.write_str("none"),
            InputMode::Text => f.write_str("text"),
            InputMode::Tel => f.write_str("tel"),
            InputMode::Url => f.write_str("url"),
            InputMode::Email => f.write_str("email"),
            InputMode::Numeric => f.write_str("numeric"),
            InputMode::Decimal => f.write_str("decimal"),
            InputMode::Search => f.write_str("search"),
        }
    }
}

/// An element's focus behaviour.
#[derive(Hash, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum TabIndex {
    /// Request that an element is to be click focusable but not sequentially focusable (-1).
    Skip,

    /// Request that an element is to be click and sequentially focusable. Focus-order should be
    /// based on the given number from lowest to highest.
    Order(u32),
}

impl fmt::Display for TabIndex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TabIndex::Skip => f.write_str("-1"),
            TabIndex::Order(order) => order.fmt(f),
        }
    }
}

#[derive(Hash)]
pub struct DontTranslate;

impl fmt::Display for DontTranslate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("no")
    }
}
