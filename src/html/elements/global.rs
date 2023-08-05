use std::borrow::Cow;
use std::fmt;

use cabin_macros::Attribute;

use crate::html::attributes::WithAttribute;

pub trait Global: WithAttribute {
    /// Used by the user agent as a guide for creating a keyboard shortcut that activates or
    /// focuses the element.
    fn access_key(self, access_key: impl Into<Cow<'static, str>>) -> Self::Output<AccessKey> {
        self.with_attribute(AccessKey(access_key.into()))
    }

    /// Hints the user-agent of how to automatically capitalize input (from non-physical
    fn auto_capitalize(self, auto_capitalize: AutoCapitalize) -> Self::Output<AutoCapitalize> {
        self.with_attribute(auto_capitalize)
    }

    /// Indicate that the element is to be focused as soon as the page is loaded.
    fn auto_focus(self) -> Self::Output<AutoFocus> {
        self.with_auto_focus(true)
    }

    /// Indicate that the element is to be focused as soon as the page is loaded.
    fn with_auto_focus(self, auto_focus: bool) -> Self::Output<AutoFocus> {
        self.with_attribute(AutoFocus(auto_focus))
    }

    /// Indicating whether the element should be editable by the user.
    fn content_editable(self) -> Self::Output<ContentEditable> {
        self.with_content_editable(true)
    }

    /// Indicating whether the element should be editable by the user.
    fn with_content_editable(self, content_editable: bool) -> Self::Output<ContentEditable> {
        self.with_attribute(ContentEditable(content_editable))
    }

    /// The element's text directionality.
    fn dir(self, dir: Dir) -> Self::Output<Dir> {
        self.with_attribute(dir)
    }

    /// Indicate whether the element can be dragged.
    fn draggable(self) -> Self::Output<Draggable> {
        self.with_draggable(true)
    }

    /// Indicate whether the element can be dragged.
    fn with_draggable(self, draggable: bool) -> Self::Output<Draggable> {
        self.with_attribute(Draggable(draggable))
    }

    /// Indicate the action label (or icon) to present for the enter key on virtual keyboards.
    fn enter_key_hint(self, enter_key_hint: EnterKeyHint) -> Self::Output<EnterKeyHint> {
        self.with_attribute(enter_key_hint)
    }

    /// Hide the element (visually and from screen-readers).
    fn hidden(self, hidden: Hidden) -> Self::Output<Hidden> {
        self.with_attribute(hidden)
    }

    /// Mark the element as not presently accessible (e.g. when overlayed by a loading state).
    fn inert(self) -> Self::Output<Inert> {
        self.with_inert(true)
    }

    /// Mark the element as not presently accessible (e.g. when overlayed by a loading state).
    fn with_inert(self, inert: bool) -> Self::Output<Inert> {
        self.with_attribute(Inert(inert))
    }

    /// Hint an input mechanism that would be most helpful for users entering content.
    fn input_mode(self, input_mode: InputMode) -> Self::Output<InputMode> {
        self.with_attribute(input_mode)
    }

    /// The element should behave like the defined custom element.
    fn is(self, is: impl Into<Cow<'static, str>>) -> Self::Output<Is> {
        self.with_attribute(Is(is.into()))
    }

    /// Unique, global identifier of the item (`item_scope` and `item_type` must also be defined).
    fn item_id(self, item_id: impl Into<Cow<'static, str>>) -> Self::Output<ItemId> {
        self.with_attribute(ItemId(item_id.into()))
    }

    /// A microdata name-value pair (this is the name – either a string or URL; the element's
    /// content is the value).
    fn item_prop(self, item_prop: impl Into<Cow<'static, str>>) -> Self::Output<ItemProp> {
        self.with_attribute(ItemProp(item_prop.into()))
    }

    /// List of element IDs elsewehre in the document with additional properties.
    fn item_ref(self, item_ref: impl Into<Cow<'static, str>>) -> Self::Output<ItemRef> {
        self.with_attribute(ItemRef(item_ref.into()))
    }

    /// Create a new item by scoping the the descendent properties together (`item_type` must also
    /// be defined).
    fn item_scope(self) -> Self::Output<ItemScope> {
        self.with_item_scope(true)
    }

    /// Create a new item by scoping the the descendent properties together (`item_type` must also
    /// be defined).
    fn with_item_scope(self, item_scope: bool) -> Self::Output<ItemScope> {
        self.with_attribute(ItemScope(item_scope))
    }

    /// URL of the vocabulary that will be used to define item properties in the data structure.
    fn item_type(self, item_type: impl Into<Cow<'static, str>>) -> Self::Output<ItemType> {
        self.with_attribute(ItemType(item_type.into()))
    }

    /// Primary language of the element's contents.
    fn lang(self, lang: impl Into<Cow<'static, str>>) -> Self::Output<Lang> {
        self.with_attribute(Lang(lang.into()))
    }

    /// Cryptographic nonce ("number used once") which can be used by Content Security Policy to
    /// determine whether or not a given fetch will be allowed to proceed.
    fn nonce(self, nonce: impl Into<Cow<'static, str>>) -> Self::Output<Nonce> {
        self.with_attribute(Nonce(nonce.into()))
    }

    /// Don't render the element until it becomes shown, at which point it will be rendered on top
    /// of other page content.
    fn popover(self) -> Self::Output<Popover> {
        self.with_popover(true)
    }

    /// Don't render the element until it becomes shown, at which point it will be rendered on top
    /// of other page content.
    fn with_popover(self, popover: bool) -> Self::Output<Popover> {
        self.with_attribute(Popover(popover))
    }

    /// The slot name this element is assigned to.
    fn slot(self, slot: impl Into<Cow<'static, str>>) -> Self::Output<Slot> {
        self.with_attribute(Slot(slot.into()))
    }

    /// Explicitly enable or disable spelling and grammar checking for the element's contents.
    fn spellcheck(self, spellcheck: bool) -> Self::Output<Spellcheck> {
        self.with_attribute(Spellcheck(spellcheck))
    }

    /// Inline CSS.
    fn style(self, style: impl Into<Cow<'static, str>>) -> Self::Output<Style> {
        self.with_attribute(Style(style.into()))
    }

    /// Specifify how and in which order the element can be focused.
    fn tab_index(self, tab_index: TabIndex) -> Self::Output<TabIndex> {
        self.with_attribute(tab_index)
    }

    /// Advisory information for the element, such as would be appropriate for a tooltip.
    fn title(self, title: impl Into<Cow<'static, str>>) -> Self::Output<Title> {
        self.with_attribute(Title(title.into()))
    }

    /// Indicate that the contents of this element should not be translated when the page is
    /// localized.
    fn translate(self, translate: bool) -> Self::Output<Translate> {
        self.with_attribute(Translate(translate))
    }
}

/// Used by the user agent as a guide for creating a keyboard shortcut that activates or
/// focuses the element.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Attribute)]
pub struct AccessKey(pub Cow<'static, str>);

/// Indicate that the element is to be focused as soon as the page is loaded.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Attribute)]
pub struct AutoFocus(pub bool);

/// Indicating whether the element should be editable by the user.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Attribute)]
pub struct ContentEditable(pub bool);

/// Indicate whether the element can be dragged.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Attribute)]
pub struct Draggable(pub bool);

/// Mark the element as not presently accessible (e.g. when overlayed by a loading state).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Attribute)]
pub struct Inert(pub bool);

/// The element should behave like the defined custom element.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Attribute)]
pub struct Is(pub Cow<'static, str>);

/// Unique, global identifier of the item (`item_scope` and `item_type` must also be defined).
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Attribute)]
pub struct ItemId(pub Cow<'static, str>);

/// A microdata name-value pair (this is the name – either a string or URL; the element's
/// content is the value).
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Attribute)]
pub struct ItemProp(pub Cow<'static, str>);

/// List of element IDs elsewehre in the document with additional properties.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Attribute)]
pub struct ItemRef(pub Cow<'static, str>);

/// Create a new item by scoping the the descendent properties together (`item_type` must also
/// be defined).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Attribute)]
pub struct ItemScope(pub bool);

/// URL of the vocabulary that will be used to define item properties in the data structure.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Attribute)]
pub struct ItemType(pub Cow<'static, str>);

/// Primary language of the element's contents.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Attribute)]
pub struct Lang(pub Cow<'static, str>);

/// Cryptographic nonce ("number used once") which can be used by Content Security Policy to
/// determine whether or not a given fetch will be allowed to proceed.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Attribute)]
pub struct Nonce(pub Cow<'static, str>);

/// Don't render the element until it becomes shown, at which point it will be rendered on top
/// of other page content.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Attribute)]
pub struct Popover(pub bool);

/// The slot name this element is assigned to.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Attribute)]
pub struct Slot(pub Cow<'static, str>);

/// Explicitly enable or disable spelling and grammar checking for the element's contents.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Attribute)]
pub struct Spellcheck(pub bool);

/// Inline CSS.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Attribute)]
pub struct Style(pub Cow<'static, str>);

/// Advisory information for the element, such as would be appropriate for a tooltip.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Attribute)]
pub struct Title(pub Cow<'static, str>);

/// Hints the user-agent of how to automatically capitalize input (from non-physical
#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Attribute)]
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
            Self::None => f.write_str("none"),
            Self::Sentences => f.write_str("sentences"),
            Self::Words => f.write_str("words"),
            Self::Characters => f.write_str("characters"),
        }
    }
}

/// The element's text directionality.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Attribute)]
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
            Self::Ltr => f.write_str("ltr"),
            Self::Rtl => f.write_str("rtl"),
            Self::Auto => f.write_str("auto"),
        }
    }
}

/// Indicate the action label (or icon) to present for the enter key on virtual keyboards.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Attribute)]
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
            Self::Enter => f.write_str("enter"),
            Self::Done => f.write_str("done"),
            Self::Go => f.write_str("go"),
            Self::Next => f.write_str("next"),
            Self::Previous => f.write_str("previous"),
            Self::Search => f.write_str("search"),
            Self::Send => f.write_str("send"),
        }
    }
}

/// Hide the element (visually and from screen-readers).
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Attribute)]
pub enum Hidden {
    /// Do not render element.
    Hidden,

    /// Do not render, unless found by searching the page or via fragment navigation.
    UntilFound,
}

impl fmt::Display for Hidden {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Hidden => f.write_str("hidden"),
            Self::UntilFound => f.write_str("until-found"),
        }
    }
}

/// Hint an input mechanism that would be most helpful for users entering content.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Attribute)]
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
            Self::None => f.write_str("none"),
            Self::Text => f.write_str("text"),
            Self::Tel => f.write_str("tel"),
            Self::Url => f.write_str("url"),
            Self::Email => f.write_str("email"),
            Self::Numeric => f.write_str("numeric"),
            Self::Decimal => f.write_str("decimal"),
            Self::Search => f.write_str("search"),
        }
    }
}

/// Specifify how and in which order the element can be focused.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Attribute)]
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
            Self::Skip => f.write_str("-1"),
            Self::Order(order) => order.fmt(f),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Attribute)]
#[attribute(outer)]
pub struct Translate(bool);

impl fmt::Display for Translate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if !self.0 {
            f.write_str("no")?;
        }
        Ok(())
    }
}
