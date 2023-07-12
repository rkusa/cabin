use std::borrow::Cow;
use std::fmt;
use std::hash::{Hash, Hasher};
use std::ops::{Deref, DerefMut};

use cabin_macros::Attributes;
use cabin_macros::{Attributes2, Element};

use crate::html::attributes::{Attributes2, Pair};

#[derive(Default, Attributes)]
pub struct AriaAttributes {
    /// Identifies the currently active element when DOM focus is on a composite widget, combobox,
    /// textbox, group, or application.
    ///
    /// <https://w3c.github.io/aria/#aria-activedescendant>
    #[attributes(attribute_name = "aria-activedescendant")]
    aria_active_descendant: Option<Cow<'static, str>>,

    /// Indicates whether assistive technologies will present all, or only parts of, the changed
    /// region based on the change notifications defined by the aria-relevant attribute.
    ///
    /// <https://w3c.github.io/aria/#aria-atomic>
    #[attributes(attribute_name = "aria-atomic")]
    aria_atomic: Option<bool>,

    /// Indicates whether inputting text could trigger display of one or more predictions of the
    /// user's intended value for a combobox, searchbox, or textbox and specifies how predictions
    /// would be presented if they were made.
    ///
    /// <https://w3c.github.io/aria/#aria-autocomplete>
    #[attributes(attribute_name = "aria-autocomplete")]
    aria_autocomplete: Option<AutoAutocomplete>,

    /// Defines a string value that labels the current element, which is intended to be converted
    /// into Braille. See related aria-label.
    ///
    /// <https://w3c.github.io/aria/#aria-braillelabel>
    #[attributes(attribute_name = "aria-braillelabel")]
    aria_braille_label: Option<Cow<'static, str>>,

    /// Defines a human-readable, author-localized abbreviated description for the role of an
    /// element, which is intended to be converted into Braille. See related aria-roledescription.
    ///
    /// <https://w3c.github.io/aria/#aria-brailleroledescription>
    #[attributes(attribute_name = "aria-brailleroledescription")]
    aria_braille_role_description: Option<Cow<'static, str>>,

    /// Indicates an element is being modified and that assistive technologies could wait until the
    /// modifications are complete before exposing them to the user.
    ///
    /// <https://w3c.github.io/aria/#aria-busy>
    #[attributes(attribute_name = "aria-busy")]
    aria_busy: Option<bool>,

    /// Indicates the current "checked" state of checkboxes, radio buttons, and other widgets. See
    /// related aria-pressed and aria-selected.
    ///
    /// <https://w3c.github.io/aria/#aria-checked>
    #[attributes(attribute_name = "aria-checked")]
    aria_checked: Option<AriaChecked>,

    /// Defines the total number of columns in a table, grid, or treegrid. See related
    /// aria-colindex.
    ///
    /// <https://w3c.github.io/aria/#aria-colcount>
    #[attributes(attribute_name = "aria-colcount")]
    aria_col_count: Option<i32>,

    /// Defines an element's column index or position with respect to the total number of columns
    /// within a table, grid, or treegrid. See related aria-colindextext, aria-colcount, and
    /// aria-colspan.
    ///
    /// <https://w3c.github.io/aria/#aria-colindex>
    #[attributes(attribute_name = "aria-colindex")]
    aria_col_index: Option<u32>,

    /// Defines a human readable text alternative of aria-colindex. See related aria-rowindextext.
    ///
    /// <https://w3c.github.io/aria/#aria-colindextext>
    #[attributes(attribute_name = "aria-colindextext")]
    aria_colindextext: Option<Cow<'static, str>>,

    /// Defines the number of columns spanned by a cell or gridcell within a table, grid, or
    /// treegrid. See related aria-colindex and aria-rowspan.
    ///
    /// <https://w3c.github.io/aria/#aria-colspan>
    #[attributes(attribute_name = "aria-colspan")]
    aria_col_span: Option<u32>,

    /// Identifies the element (or elements) whose contents or presence are controlled by the
    /// current element. See related aria-owns.
    ///
    /// <https://w3c.github.io/aria/#aria-controls>
    #[attributes(attribute_name = "aria-controls")]
    aria_controls: Option<Cow<'static, str>>,

    /// Indicates the element that represents the current item within a container or set of related
    /// elements.
    ///
    /// <https://w3c.github.io/aria/#aria-current>
    #[attributes(attribute_name = "aria-current")]
    aria_current: Option<AriaCurrent>,

    /// Identifies the element (or elements) that describes the object. See related aria-labelledby
    /// and aria-description.
    ///
    /// <https://w3c.github.io/aria/#aria-describedby>
    #[attributes(attribute_name = "aria-describedby")]
    aria_describedby: Option<Cow<'static, str>>,

    /// Defines a string value that describes or annotates the current element. See related
    /// aria-describedby.
    ///
    /// <https://w3c.github.io/aria/#aria-description>
    #[attributes(attribute_name = "aria-description")]
    aria_description: Option<Cow<'static, str>>,

    /// Identifies the element (or elements) that provide additional information related to the
    /// object. See related aria-describedby.
    ///
    /// <https://w3c.github.io/aria/#aria-details>
    #[attributes(attribute_name = "aria-details")]
    aria_details: Option<Cow<'static, str>>,

    /// Indicates that the element is perceivable but disabled, so it is not editable or otherwise
    /// operable. See related aria-hidden and aria-readonly.
    ///
    /// <https://w3c.github.io/aria/#aria-disabled>
    #[attributes(attribute_name = "aria-disabled")]
    aria_disabled: Option<bool>,

    /// Identifies the element (or elements) that provides an error message for an object. See
    /// related aria-invalid and aria-describedby.
    ///
    /// <https://w3c.github.io/aria/#aria-errormessage>
    #[attributes(attribute_name = "aria-errormessage")]
    aria_error_message: Option<Cow<'static, str>>,

    /// Indicates whether a grouping element owned or controlled by this element is expanded or
    /// collapsed.
    ///
    /// <https://w3c.github.io/aria/#aria-expanded>
    #[attributes(attribute_name = "aria-expanded")]
    aria_expanded: Option<bool>,

    /// Identifies the next element (or elements) in an alternate reading order of content which,
    /// at the user's discretion, allows assistive technology to override the general default of
    /// reading in document source order.
    ///
    /// <https://w3c.github.io/aria/#aria-flowto>
    #[attributes(attribute_name = "aria-flowto")]
    aria_flow_to: Option<Cow<'static, str>>,

    /// Indicates the availability and type of interactive popup element, such as menu or dialog,
    /// that can be triggered by an element.
    ///
    /// <https://w3c.github.io/aria/#aria-haspopup>
    #[attributes(attribute_name = "aria-haspopup")]
    aria_haspopup: Option<AriaHasPopup>,

    /// Indicates whether the element is exposed to an accessibility API. See related
    /// aria-disabled.
    ///
    /// <https://w3c.github.io/aria/#aria-hidden>
    #[attributes(attribute_name = "aria-hidden")]
    aria_hidden: Option<bool>,

    /// Indicates the entered value does not conform to the format expected by the application.
    /// See related aria-errormessage.
    ///
    /// <https://w3c.github.io/aria/#aria-invalid>
    #[attributes(attribute_name = "aria-invalid")]
    aria_invalid: Option<AriaInvalid>,

    /// Defines keyboard shortcuts that an author has implemented to activate or give focus to an
    /// element.
    ///
    /// <https://w3c.github.io/aria/#aria-keyshortcuts>
    #[attributes(attribute_name = "aria-keyshortcuts")]
    aria_key_shortcuts: Option<Cow<'static, str>>,

    /// Defines a string value that labels the current element. See related aria-labelledby.
    ///
    /// <https://w3c.github.io/aria/#aria-label>
    #[attributes(attribute_name = "aria-label")]
    aria_label: Option<Cow<'static, str>>,

    /// Identifies the element (or elements) that labels the current element. See related
    /// aria-label and aria-describedby.
    ///
    /// <https://w3c.github.io/aria/#aria-labelledby>
    #[attributes(attribute_name = "aria-labelledby")]
    aria_labelledby: Option<Cow<'static, str>>,

    /// Defines the hierarchical level of an element within a structure.
    ///
    /// <https://w3c.github.io/aria/#aria-level>
    #[attributes(attribute_name = "aria-level")]
    aria_level: Option<u32>,

    /// Indicates that an element will be updated, and describes the types of updates the user
    /// agents, assistive technologies, and user can expect from the live region.
    ///
    /// <https://w3c.github.io/aria/#aria-live>
    #[attributes(attribute_name = "aria-live")]
    aria_live: Option<AriaLive>,

    /// Indicates whether an element is modal when displayed.
    ///
    /// <https://w3c.github.io/aria/#aria-modal>
    #[attributes(attribute_name = "aria-modal")]
    aria_modal: Option<bool>,

    /// Indicates whether a text box accepts multiple lines of input or only a single line.
    ///
    /// <https://w3c.github.io/aria/#aria-multiline>
    #[attributes(attribute_name = "aria-multiline")]
    aria_multi_line: Option<bool>,

    /// Indicates that the user can select more than one item from the current selectable
    /// descendants.
    ///
    /// <https://w3c.github.io/aria/#aria-multiselectable>
    #[attributes(attribute_name = "aria-multiselectable")]
    aria_multi_selectable: Option<bool>,

    /// Indicates whether the element's orientation is horizontal, vertical, or unknown/ambiguous.
    ///
    /// <https://w3c.github.io/aria/#aria-orientation>
    #[attributes(attribute_name = "aria-orientation")]
    aria_orientation: Option<AriaOrientation>,

    /// Identifies an element (or elements) in order to define a visual, functional, or contextual
    /// parent/child relationship between DOM elements where the DOM hierarchy cannot be used to
    /// represent the relationship. See related aria-controls.
    ///
    /// <https://w3c.github.io/aria/#aria-owns>
    #[attributes(attribute_name = "aria-owns")]
    aria_owns: Option<Cow<'static, str>>,

    /// Defines a short hint (a word or short phrase) intended to aid the user with data entry when
    /// the control has no value. A hint could be a sample value or a brief description of the
    /// expected format.
    ///
    /// <https://w3c.github.io/aria/#aria-placeholder>
    #[attributes(attribute_name = "aria-placeholder")]
    aria_placeholder: Option<Cow<'static, str>>,

    /// Defines an element's number or position in the current set of listitems or treeitems. Not
    /// required if all elements in the set are present in the DOM. See related aria-setsize.
    ///
    /// <https://w3c.github.io/aria/#aria-posinset>
    #[attributes(attribute_name = "aria-posinset")]
    aria_pos_inset: Option<u32>,

    /// Indicates the current "pressed" state of toggle buttons. See related aria-checked and
    /// aria-selected.
    ///
    /// <https://w3c.github.io/aria/#aria-pressed>
    #[attributes(attribute_name = "aria-pressed")]
    aria_pressed: Option<AriaPressed>,

    /// Indicates that the element is not editable, but is otherwise operable. See related
    /// aria-disabled.
    ///
    /// <https://w3c.github.io/aria/#aria-readonly>
    #[attributes(attribute_name = "aria-readonly")]
    aria_readonly: Option<bool>,

    /// Indicates what notifications the user agent will trigger when the accessibility tree within
    /// a live region is modified. See related aria-atomic.
    ///
    /// <https://w3c.github.io/aria/#aria-relevant>
    #[attributes(attribute_name = "aria-relevant")]
    aria_relevant: Option<AriaRelevant>,

    /// Indicates that user input is required on the element before a form can be submitted.
    ///
    /// <https://w3c.github.io/aria/#aria-required>
    #[attributes(attribute_name = "aria-required")]
    aria_required: Option<bool>,

    /// Defines a human-readable, author-localized description for the role of an element.
    ///
    /// <https://w3c.github.io/aria/#aria-roledescription>
    #[attributes(attribute_name = "aria-roledescription")]
    aria_role_description: Option<Cow<'static, str>>,

    /// Defines the total number of rows in a table, grid, or treegrid. See related aria-rowindex.
    ///
    /// <https://w3c.github.io/aria/#aria-rowcount>
    #[attributes(attribute_name = "aria-rowcount")]
    aria_row_count: Option<i32>,

    /// Defines an element's row index or position with respect to the total number of rows within
    /// a table, grid, or treegrid. See related aria-rowindextext, aria-rowcount, and aria-rowspan.
    ///
    /// <https://w3c.github.io/aria/#aria-rowindex>
    #[attributes(attribute_name = "aria-rowindex")]
    aria_row_index: Option<u32>,

    /// Defines a human readable text alternative of aria-rowindex. See related aria-colindextext.
    ///
    /// <https://w3c.github.io/aria/#aria-rowindextext>
    #[attributes(attribute_name = "aria-rowindextext")]
    aria_row_index_text: Option<Cow<'static, str>>,

    /// Defines the number of rows spanned by a cell or gridcell within a table, grid, or treegrid.
    /// See related aria-rowindex and aria-colspan.
    ///
    /// <https://w3c.github.io/aria/#aria-rowspan>
    #[attributes(attribute_name = "aria-rowspan")]
    aria_row_span: Option<u32>,

    /// Indicates the current "selected" state of various widgets. See related aria-checked and
    /// aria-pressed.
    ///
    /// <https://w3c.github.io/aria/#aria-selected>
    #[attributes(attribute_name = "aria-selected")]
    aria_selected: Option<bool>,

    /// Defines the number of items in the current set of listitems or treeitems. Not required if
    /// all elements in the set are present in the DOM. See related aria-posinset.
    ///
    /// <https://w3c.github.io/aria/#aria-setsize>
    #[attributes(attribute_name = "aria-setsize")]
    aria_set_size: Option<i32>,

    /// Indicates if items in a table or grid are sorted in ascending or descending order.
    ///
    /// <https://w3c.github.io/aria/#aria-sort>
    #[attributes(attribute_name = "aria-sort")]
    aria_sort: Option<AriaSort>,

    /// Defines the maximum allowed value for a range widget.
    ///
    /// <https://w3c.github.io/aria/#aria-valuemax>
    #[attributes(attribute_name = "aria-valuemax")]
    aria_value_max: Option<Number>,

    /// Defines the minimum allowed value for a range widget.
    ///
    /// <https://w3c.github.io/aria/#aria-valuemin>
    #[attributes(attribute_name = "aria-valuemin")]
    aria_value_min: Option<Number>,

    /// Defines the current value for a range widget. See related aria-valuetext.
    ///
    /// <https://w3c.github.io/aria/#aria-valuenow>
    #[attributes(attribute_name = "aria-valuenow")]
    aria_value_now: Option<Number>,

    /// Defines the human readable text alternative of aria-valuenow for a range widget.
    ///
    /// <https://w3c.github.io/aria/#aria-valuetext>
    #[attributes(attribute_name = "aria-valuetext")]
    aria_value_text: Option<Cow<'static, str>>,
}

#[derive(Hash, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum AutoAutocomplete {
    /// When a user is providing input, text suggesting one way to complete the provided input
    /// might be dynamically inserted after the caret.
    Inline,

    /// When a user is providing input, an element containing a collection of values that could
    /// complete the provided input might be displayed.
    List,

    /// When a user is providing input, an element containing a collection of values that could
    /// complete the provided input might be displayed. If displayed, one value in the collection
    /// is automatically selected, and the text needed to complete the automatically selected value
    /// appears after the caret in the input.
    Both,

    /// When a user is providing input, an automatic suggestion that attempts to predict how the
    /// user intends to complete the input is not displayed.
    None,
}

impl fmt::Display for AutoAutocomplete {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AutoAutocomplete::Inline => f.write_str("inline"),
            AutoAutocomplete::List => f.write_str("list"),
            AutoAutocomplete::Both => f.write_str("both"),
            AutoAutocomplete::None => f.write_str("none"),
        }
    }
}

#[derive(Hash, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum AriaChecked {
    /// The element is checked.
    True,

    /// The element supports being checked but is not currently checked.
    False,

    /// Indicates a mixed mode value for a tri-state checkbox or menuitemcheckbox.
    Mixed,
}

impl fmt::Display for AriaChecked {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AriaChecked::True => f.write_str("true"),
            AriaChecked::False => f.write_str("false"),
            AriaChecked::Mixed => f.write_str("mixed"),
        }
    }
}

#[derive(Hash, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum AriaCurrent {
    /// Represents the current page within a set of pages.
    Page,

    /// Represents the current step within a process.
    Step,

    /// Represents the current location within an environment or context.
    Location,

    /// Represents the current date within a collection of dates.
    Date,

    /// Represents the current time within a set of times.
    Time,

    /// Represents the current item within a set.
    True,

    /// Does not represent the current item within a set.
    False,
}

impl fmt::Display for AriaCurrent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AriaCurrent::Page => f.write_str("page"),
            AriaCurrent::Step => f.write_str("step"),
            AriaCurrent::Location => f.write_str("location"),
            AriaCurrent::Date => f.write_str("date"),
            AriaCurrent::Time => f.write_str("time"),
            AriaCurrent::True => f.write_str("true"),
            AriaCurrent::False => f.write_str("false"),
        }
    }
}

#[derive(Hash, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum AriaHasPopup {
    /// Indicates the element does not have a popup.
    False,

    /// Indicates the popup is a menu.
    True,

    /// Indicates the popup is a menu.
    Menu,

    /// Indicates the popup is a listbox.
    Listbox,

    /// Indicates the popup is a tree.
    Tree,

    /// Indicates the popup is a grid.
    Grid,

    /// Indicates the popup is a dialog.
    Dialog,
}

impl fmt::Display for AriaHasPopup {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AriaHasPopup::False => f.write_str("false"),
            AriaHasPopup::True => f.write_str("true"),
            AriaHasPopup::Menu => f.write_str("menu"),
            AriaHasPopup::Listbox => f.write_str("listbox"),
            AriaHasPopup::Tree => f.write_str("tree"),
            AriaHasPopup::Grid => f.write_str("grid"),
            AriaHasPopup::Dialog => f.write_str("dialog"),
        }
    }
}

#[derive(Hash, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum AriaInvalid {
    /// A grammatical error was detected.
    Grammar,

    /// There are no detected errors in the value.
    False,

    /// A spelling error was detected.
    Spelling,

    /// The value entered by the user has failed validation.
    True,
}

impl fmt::Display for AriaInvalid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AriaInvalid::Grammar => f.write_str("grammar"),
            AriaInvalid::False => f.write_str("false"),
            AriaInvalid::Spelling => f.write_str("spelling"),
            AriaInvalid::True => f.write_str("true"),
        }
    }
}

#[derive(Hash, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum AriaLive {
    /// Indicates that updates to the region have the highest priority and should be presented
    /// the user immediately.
    Assertive,

    /// Indicates that updates to the region should not be presented to the user unless the user is
    /// currently focused on that region.
    Off,

    /// Indicates that updates to the region should be presented at the next graceful opportunity,
    /// such as at the end of speaking the current sentence or when the user pauses typing.
    Polite,
}

impl fmt::Display for AriaLive {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AriaLive::Assertive => f.write_str("assertive"),
            AriaLive::Off => f.write_str("off"),
            AriaLive::Polite => f.write_str("polite"),
        }
    }
}

#[derive(Hash, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum AriaOrientation {
    /// The element is oriented horizontally.
    Horizontal,

    /// The element is oriented vertically.
    Vertical,
}

impl fmt::Display for AriaOrientation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AriaOrientation::Horizontal => f.write_str("horizontal"),
            AriaOrientation::Vertical => f.write_str("vertical"),
        }
    }
}

#[derive(Hash, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum AriaPressed {
    /// The element is pressed.
    True,

    /// The element supports being pressed but is not currently pressed.
    False,

    /// Indicates a mixed mode value for a tri-state toggle button.
    Mixed,
}

impl fmt::Display for AriaPressed {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AriaPressed::True => f.write_str("true"),
            AriaPressed::False => f.write_str("false"),
            AriaPressed::Mixed => f.write_str("mixed"),
        }
    }
}

#[derive(Hash, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum AriaRelevant {
    /// Element nodes are added to the accessibility tree within the live region.
    Additions,

    /// Equivalent to the combination of values, "additions text".
    AdditionsText,

    /// Equivalent to the combination of all values, "additions removals text".
    All,

    /// Text content, a text alternative, or an element node within the live region is removed from
    /// the accessibility tree.
    Removals,

    /// Text content or a text alternative is added to any descendant in the accessibility tree of
    /// the live region.
    Text,
}

impl fmt::Display for AriaRelevant {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AriaRelevant::Additions => f.write_str("additions"),
            AriaRelevant::AdditionsText => f.write_str("additions text"),
            AriaRelevant::All => f.write_str("all"),
            AriaRelevant::Removals => f.write_str("removals"),
            AriaRelevant::Text => f.write_str("text"),
        }
    }
}

#[derive(Hash, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum AriaSort {
    /// Items are sorted in ascending order.
    Ascending,

    /// Items are sorted in descending order.
    Descending,

    /// A sort algorithm other than ascending or descending has been applied.
    Other,
}

impl fmt::Display for AriaSort {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AriaSort::Ascending => f.write_str("ascending"),
            AriaSort::Descending => f.write_str("descending"),
            AriaSort::Other => f.write_str("other"),
        }
    }
}

/// Hashable f64 (hashed with precision of 6).
#[derive(PartialEq, PartialOrd, Clone, Copy)]
pub struct Number(pub f64);

impl Deref for Number {
    type Target = f64;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Number {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Hash for Number {
    fn hash<H: Hasher>(&self, state: &mut H) {
        ((self.0) as i64).hash(state);
        ((self.0 * 1000000.0) as i64).hash(state);
    }
}

impl<T> From<T> for Number
where
    f64: From<T>,
{
    fn from(value: T) -> Self {
        Number(f64::from(value))
    }
}

impl fmt::Display for Number {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}
