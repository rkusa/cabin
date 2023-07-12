use std::borrow::Cow;
use std::fmt;
use std::hash::{Hash, Hasher};
use std::ops::{Deref, DerefMut};

use cabin_macros::{element, Attribute};

use crate::html::attributes::{Attributes, Pair};

#[element(tag = false)]
pub trait Aria: Attributes {
    /// Identifies the currently active element when DOM focus is on a composite widget, combobox,
    /// textbox, group, or application.
    ///
    /// <https://w3c.github.io/aria/#aria-activedescendant>
    fn aria_active_descendant(
        self,
        aria_active_descendant: impl Into<Cow<'static, str>>,
    ) -> impl Aria {
        self.with(AriaActiveDescendant(aria_active_descendant.into()))
    }

    /// Indicates whether assistive technologies will present all, or only parts of, the changed
    /// region based on the change notifications defined by the aria-relevant attribute.
    ///
    /// <https://w3c.github.io/aria/#aria-atomic>
    fn aria_atomic(self, aria_atomic: bool) -> impl Aria {
        self.with(AriaAtomic(aria_atomic))
    }

    /// Indicates whether inputting text could trigger display of one or more predictions of the
    /// user's intended value for a combobox, searchbox, or textbox and specifies how predictions
    /// would be presented if they were made.
    ///
    /// <https://w3c.github.io/aria/#aria-autocomplete>
    fn aria_autocomplete(self, aria_autocomplete: AutoAutocomplete) -> impl Aria {
        self.with(aria_autocomplete)
    }

    /// Defines a string value that labels the current element, which is intended to be converted
    /// into Braille. See related aria-label.
    ///
    /// <https://w3c.github.io/aria/#aria-braillelabel>
    fn aria_braille_label(self, aria_braille_label: impl Into<Cow<'static, str>>) -> impl Aria {
        self.with(AriaBrailleLabel(aria_braille_label.into()))
    }

    /// Defines a human-readable, author-localized abbreviated description for the role of an
    /// element, which is intended to be converted into Braille. See related aria-roledescription.
    ///
    /// <https://w3c.github.io/aria/#aria-brailleroledescription>
    fn aria_braille_role_description(
        self,
        aria_braille_role_description: impl Into<Cow<'static, str>>,
    ) -> impl Aria {
        self.with(AriaBrailleRoleDescription(
            aria_braille_role_description.into(),
        ))
    }

    /// Indicates an element is being modified and that assistive technologies could wait until the
    /// modifications are complete before exposing them to the user.
    ///
    /// <https://w3c.github.io/aria/#aria-busy>
    fn aria_busy(self, aria_busy: bool) -> impl Aria {
        self.with(AriaBusy(aria_busy))
    }

    /// Indicates the current "checked" state of checkboxes, radio buttons, and other widgets. See
    /// related aria-pressed and aria-selected.
    ///
    /// <https://w3c.github.io/aria/#aria-checked>
    fn aria_checked(self, aria_checked: AriaChecked) -> impl Aria {
        self.with(aria_checked)
    }

    /// Defines the total number of columns in a table, grid, or treegrid. See related
    /// aria-colindex.
    ///
    /// <https://w3c.github.io/aria/#aria-colcount>
    fn aria_col_count(self, aria_col_count: impl Into<i32>) -> impl Aria {
        self.with(AriaColCount(aria_col_count.into()))
    }

    /// Defines an element's column index or position with respect to the total number of columns
    /// within a table, grid, or treegrid. See related aria-colindextext, aria-colcount, and
    /// aria-colspan.
    ///
    /// <https://w3c.github.io/aria/#aria-colindex>
    fn aria_col_index(self, aria_col_index: impl Into<u32>) -> impl Aria {
        self.with(AriaColIndex(aria_col_index.into()))
    }

    /// Defines a human readable text alternative of aria-colindex. See related aria-rowindextext.
    ///
    /// <https://w3c.github.io/aria/#aria-colindextext>
    fn aria_colindextext(self, aria_colindextext: impl Into<Cow<'static, str>>) -> impl Aria {
        self.with(AriaColindextext(aria_colindextext.into()))
    }

    /// Defines the number of columns spanned by a cell or gridcell within a table, grid, or
    /// treegrid. See related aria-colindex and aria-rowspan.
    ///
    /// <https://w3c.github.io/aria/#aria-colspan>
    fn aria_col_span(self, aria_col_span: impl Into<u32>) -> impl Aria {
        self.with(AriaColSpan(aria_col_span.into()))
    }

    /// Identifies the element (or elements) whose contents or presence are controlled by the
    /// current element. See related aria-owns.
    ///
    /// <https://w3c.github.io/aria/#aria-controls>
    fn aria_controls(self, aria_controls: impl Into<Cow<'static, str>>) -> impl Aria {
        self.with(AriaControls(aria_controls.into()))
    }

    /// Indicates the element that represents the current item within a container or set of related
    /// elements.
    ///
    /// <https://w3c.github.io/aria/#aria-current>
    fn aria_current(self, aria_current: AriaCurrent) -> impl Aria {
        self.with(aria_current)
    }

    /// Identifies the element (or elements) that describes the object. See related aria-labelledby
    /// and aria-description.
    ///
    /// <https://w3c.github.io/aria/#aria-describedby>
    fn aria_describedby(self, aria_describedby: impl Into<Cow<'static, str>>) -> impl Aria {
        self.with(AriaDescribedby(aria_describedby.into()))
    }

    /// Defines a string value that describes or annotates the current element. See related
    /// aria-describedby.
    ///
    /// <https://w3c.github.io/aria/#aria-description>
    fn aria_description(self, aria_description: impl Into<Cow<'static, str>>) -> impl Aria {
        self.with(AriaDescription(aria_description.into()))
    }

    /// Identifies the element (or elements) that provide additional information related to the
    /// object. See related aria-describedby.
    ///
    /// <https://w3c.github.io/aria/#aria-details>
    fn aria_details(self, aria_details: impl Into<Cow<'static, str>>) -> impl Aria {
        self.with(AriaDetails(aria_details.into()))
    }

    /// Indicates that the element is perceivable but disabled, so it is not editable or otherwise
    /// operable. See related aria-hidden and aria-readonly.
    ///
    /// <https://w3c.github.io/aria/#aria-disabled>
    fn aria_disabled(self, aria_disabled: bool) -> impl Aria {
        self.with(AriaDisabled(aria_disabled))
    }

    /// Identifies the element (or elements) that provides an error message for an object. See
    /// related aria-invalid and aria-describedby.
    ///
    /// <https://w3c.github.io/aria/#aria-errormessage>
    fn aria_error_message(self, aria_error_message: impl Into<Cow<'static, str>>) -> impl Aria {
        self.with(AriaErrorMessage(aria_error_message.into()))
    }

    /// Indicates whether a grouping element owned or controlled by this element is expanded or
    /// collapsed.
    ///
    /// <https://w3c.github.io/aria/#aria-expanded>
    fn aria_expanded(self, aria_expanded: bool) -> impl Aria {
        self.with(AriaExpanded(aria_expanded))
    }

    /// Identifies the next element (or elements) in an alternate reading order of content which,
    /// at the user's discretion, allows assistive technology to override the general default of
    /// reading in document source order.
    ///
    /// <https://w3c.github.io/aria/#aria-flowto>
    fn aria_flow_to(self, aria_flow_to: impl Into<Cow<'static, str>>) -> impl Aria {
        self.with(AriaFlowTo(aria_flow_to.into()))
    }

    /// Indicates the availability and type of interactive popup element, such as menu or dialog,
    /// that can be triggered by an element.
    ///
    /// <https://w3c.github.io/aria/#aria-haspopup>
    fn aria_haspopup(self, aria_haspopup: AriaHasPopup) -> impl Aria {
        self.with(aria_haspopup)
    }

    /// Indicates whether the element is exposed to an accessibility API. See related
    /// aria-disabled.
    ///
    /// <https://w3c.github.io/aria/#aria-hidden>
    fn aria_hidden(self, aria_hidden: bool) -> impl Aria {
        self.with(AriaHidden(aria_hidden))
    }

    /// Indicates the entered value does not conform to the format expected by the application.
    /// See related aria-errormessage.
    ///
    /// <https://w3c.github.io/aria/#aria-invalid>
    fn aria_invalid(self, aria_invalid: AriaInvalid) -> impl Aria {
        self.with(aria_invalid)
    }

    /// Defines keyboard shortcuts that an author has implemented to activate or give focus to an
    /// element.
    ///
    /// <https://w3c.github.io/aria/#aria-keyshortcuts>
    fn aria_key_shortcuts(self, aria_key_shortcuts: impl Into<Cow<'static, str>>) -> impl Aria {
        self.with(AriaKeyShortcuts(aria_key_shortcuts.into()))
    }

    /// Defines a string value that labels the current element. See related aria-labelledby.
    ///
    /// <https://w3c.github.io/aria/#aria-label>
    fn aria_label(self, aria_label: impl Into<Cow<'static, str>>) -> impl Aria {
        self.with(AriaLabel(aria_label.into()))
    }

    /// Identifies the element (or elements) that labels the current element. See related
    /// aria-label and aria-describedby.
    ///
    /// <https://w3c.github.io/aria/#aria-labelledby>
    fn aria_labelledby(self, aria_labelledby: impl Into<Cow<'static, str>>) -> impl Aria {
        self.with(AriaLabelledby(aria_labelledby.into()))
    }

    /// Defines the hierarchical level of an element within a structure.
    ///
    /// <https://w3c.github.io/aria/#aria-level>
    fn aria_level(self, aria_level: impl Into<u32>) -> impl Aria {
        self.with(AriaLevel(aria_level.into()))
    }

    /// Indicates that an element will be updated, and describes the types of updates the user
    /// agents, assistive technologies, and user can expect from the live region.
    ///
    /// <https://w3c.github.io/aria/#aria-live>
    fn aria_live(self, aria_live: AriaLive) -> impl Aria {
        self.with(aria_live)
    }

    /// Indicates whether an element is modal when displayed.
    ///
    /// <https://w3c.github.io/aria/#aria-modal>
    fn aria_modal(self, aria_modal: bool) -> impl Aria {
        self.with(AriaModal(aria_modal))
    }

    /// Indicates whether a text box accepts multiple lines of input or only a single line.
    ///
    /// <https://w3c.github.io/aria/#aria-multiline>
    fn aria_multi_line(self, aria_multi_line: bool) -> impl Aria {
        self.with(AriaMultiLine(aria_multi_line))
    }

    /// Indicates that the user can select more than one item from the current selectable
    /// descendants.
    ///
    /// <https://w3c.github.io/aria/#aria-multiselectable>
    fn aria_multi_selectable(self, aria_multi_selectable: bool) -> impl Aria {
        self.with(AriaMultiSelectable(aria_multi_selectable))
    }

    /// Indicates whether the element's orientation is horizontal, vertical, or unknown/ambiguous.
    ///
    /// <https://w3c.github.io/aria/#aria-orientation>
    fn aria_orientation(self, aria_orientation: AriaOrientation) -> impl Aria {
        self.with(aria_orientation)
    }

    /// Identifies an element (or elements) in order to define a visual, functional, or contextual
    /// parent/child relationship between DOM elements where the DOM hierarchy cannot be used to
    /// represent the relationship. See related aria-controls.
    ///
    /// <https://w3c.github.io/aria/#aria-owns>
    fn aria_owns(self, aria_owns: impl Into<Cow<'static, str>>) -> impl Aria {
        self.with(AriaOwns(aria_owns.into()))
    }

    /// Defines a short hint (a word or short phrase) intended to aid the user with data entry when
    /// the control has no value. A hint could be a sample value or a brief description of the
    /// expected format.
    ///
    /// <https://w3c.github.io/aria/#aria-placeholder>
    fn aria_placeholder(self, aria_placeholder: impl Into<Cow<'static, str>>) -> impl Aria {
        self.with(AriaPlaceholder(aria_placeholder.into()))
    }

    /// Defines an element's number or position in the current set of listitems or treeitems. Not
    /// required if all elements in the set are present in the DOM. See related aria-setsize.
    ///
    /// <https://w3c.github.io/aria/#aria-posinset>
    fn aria_pos_inset(self, aria_pos_inset: impl Into<u32>) -> impl Aria {
        self.with(AriaPosInset(aria_pos_inset.into()))
    }

    /// Indicates the current "pressed" state of toggle buttons. See related aria-checked and
    /// aria-selected.
    ///
    /// <https://w3c.github.io/aria/#aria-pressed>
    fn aria_pressed(self, aria_pressed: AriaPressed) -> impl Aria {
        self.with(aria_pressed)
    }

    /// Indicates that the element is not editable, but is otherwise operable. See related
    /// aria-disabled.
    ///
    /// <https://w3c.github.io/aria/#aria-readonly>
    fn aria_readonly(self, aria_readonly: bool) -> impl Aria {
        self.with(AriaReadonly(aria_readonly))
    }

    /// Indicates what notifications the user agent will trigger when the accessibility tree within
    /// a live region is modified. See related aria-atomic.
    ///
    /// <https://w3c.github.io/aria/#aria-relevant>
    fn aria_relevant(self, aria_relevant: AriaRelevant) -> impl Aria {
        self.with(aria_relevant)
    }

    /// Indicates that user input is required on the element before a form can be submitted.
    ///
    /// <https://w3c.github.io/aria/#aria-required>
    fn aria_required(self, aria_required: bool) -> impl Aria {
        self.with(AriaRequired(aria_required))
    }

    /// Defines a human-readable, author-localized description for the role of an element.
    ///
    /// <https://w3c.github.io/aria/#aria-roledescription>
    fn aria_role_description(
        self,
        aria_role_description: impl Into<Cow<'static, str>>,
    ) -> impl Aria {
        self.with(AriaRoleDescription(aria_role_description.into()))
    }

    /// Defines the total number of rows in a table, grid, or treegrid. See related aria-rowindex.
    ///
    /// <https://w3c.github.io/aria/#aria-rowcount>
    fn aria_row_count(self, aria_row_count: impl Into<i32>) -> impl Aria {
        self.with(AriaRowCount(aria_row_count.into()))
    }

    /// Defines an element's row index or position with respect to the total number of rows within
    /// a table, grid, or treegrid. See related aria-rowindextext, aria-rowcount, and aria-rowspan.
    ///
    /// <https://w3c.github.io/aria/#aria-rowindex>
    fn aria_row_index(self, aria_row_index: impl Into<u32>) -> impl Aria {
        self.with(AriaRowIndex(aria_row_index.into()))
    }

    /// Defines a human readable text alternative of aria-rowindex. See related aria-colindextext.
    ///
    /// <https://w3c.github.io/aria/#aria-rowindextext>
    fn aria_row_index_text(self, aria_row_index_text: impl Into<Cow<'static, str>>) -> impl Aria {
        self.with(AriaRowIndexText(aria_row_index_text.into()))
    }

    /// Defines the number of rows spanned by a cell or gridcell within a table, grid, or treegrid.
    /// See related aria-rowindex and aria-colspan.
    ///
    /// <https://w3c.github.io/aria/#aria-rowspan>
    fn aria_row_span(self, aria_row_span: impl Into<u32>) -> impl Aria {
        self.with(AriaRowSpan(aria_row_span.into()))
    }

    /// Indicates the current "selected" state of various widgets. See related aria-checked and
    /// aria-pressed.
    ///
    /// <https://w3c.github.io/aria/#aria-selected>
    fn aria_selected(self, aria_selected: bool) -> impl Aria {
        self.with(AriaSelected(aria_selected))
    }

    /// Defines the number of items in the current set of listitems or treeitems. Not required if
    /// all elements in the set are present in the DOM. See related aria-posinset.
    ///
    /// <https://w3c.github.io/aria/#aria-setsize>
    fn aria_set_size(self, aria_set_size: impl Into<i32>) -> impl Aria {
        self.with(AriaSetSize(aria_set_size.into()))
    }

    /// Indicates if items in a table or grid are sorted in ascending or descending order.
    ///
    /// <https://w3c.github.io/aria/#aria-sort>
    fn aria_sort(self, aria_sort: AriaSort) -> impl Aria {
        self.with(aria_sort)
    }

    /// Defines the maximum allowed value for a range widget.
    ///
    /// <https://w3c.github.io/aria/#aria-valuemax>
    fn aria_value_max(self, aria_value_max: impl Into<Number>) -> impl Aria {
        self.with(AriaValueMax(aria_value_max.into()))
    }

    /// Defines the minimum allowed value for a range widget.
    ///
    /// <https://w3c.github.io/aria/#aria-valuemin>
    fn aria_value_min(self, aria_value_min: impl Into<Number>) -> impl Aria {
        self.with(AriaValueMin(aria_value_min.into()))
    }

    /// Defines the current value for a range widget. See related aria-valuetext.
    ///
    /// <https://w3c.github.io/aria/#aria-valuenow>
    fn aria_value_now(self, aria_value_now: impl Into<Number>) -> impl Aria {
        self.with(AriaValueNow(aria_value_now.into()))
    }

    /// Defines the human readable text alternative of aria-valuenow for a range widget.
    ///
    /// <https://w3c.github.io/aria/#aria-valuetext>
    fn aria_value_text(self, aria_value_text: impl Into<Cow<'static, str>>) -> impl Aria {
        self.with(AriaValueText(aria_value_text.into()))
    }
}

/// Identifies the currently active element when DOM focus is on a composite widget, combobox,
/// textbox, group, or application.
///
/// <https://w3c.github.io/aria/#aria-activedescendant>
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Attribute)]
#[attribute(name = "aria-activedescendant")]
pub struct AriaActiveDescendant(pub Cow<'static, str>);

/// Indicates whether assistive technologies will present all, or only parts of, the changed
/// region based on the change notifications defined by the aria-relevant attribute.
///
/// <https://w3c.github.io/aria/#aria-atomic>
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Attribute)]
#[attribute(name = "aria-atomic")]
pub struct AriaAtomic(pub bool);

/// Defines a string value that labels the current element, which is intended to be converted
/// into Braille. See related aria-label.
///
/// <https://w3c.github.io/aria/#aria-braillelabel>
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Attribute)]
#[attribute(name = "aria-braillelabel")]
pub struct AriaBrailleLabel(pub Cow<'static, str>);

/// Defines a human-readable, author-localized abbreviated description for the role of an
/// element, which is intended to be converted into Braille. See related aria-roledescription.
///
/// <https://w3c.github.io/aria/#aria-brailleroledescription>
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Attribute)]
#[attribute(name = "aria-brailleroledescription")]
pub struct AriaBrailleRoleDescription(pub Cow<'static, str>);

/// Indicates an element is being modified and that assistive technologies could wait until the
/// modifications are complete before exposing them to the user.
///
/// <https://w3c.github.io/aria/#aria-busy>
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Attribute)]
#[attribute(name = "aria-busy")]
pub struct AriaBusy(pub bool);

/// Defines the total number of columns in a table, grid, or treegrid. See related
/// aria-colindex.
///
/// <https://w3c.github.io/aria/#aria-colcount>
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Attribute)]
#[attribute(name = "aria-colcount")]
pub struct AriaColCount(pub i32);

/// Defines an element's column index or position with respect to the total number of columns
/// within a table, grid, or treegrid. See related aria-colindextext, aria-colcount, and
/// aria-colspan.
///
/// <https://w3c.github.io/aria/#aria-colindex>
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Attribute)]
#[attribute(name = "aria-colindex")]
pub struct AriaColIndex(pub u32);

/// Defines a human readable text alternative of aria-colindex. See related aria-rowindextext.
///
/// <https://w3c.github.io/aria/#aria-colindextext>
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Attribute)]
#[attribute(name = "aria-colindextext")]
pub struct AriaColindextext(pub Cow<'static, str>);

/// Defines the number of columns spanned by a cell or gridcell within a table, grid, or
/// treegrid. See related aria-colindex and aria-rowspan.
///
/// <https://w3c.github.io/aria/#aria-colspan>
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Attribute)]
#[attribute(name = "aria-colspan")]
pub struct AriaColSpan(pub u32);

/// Identifies the element (or elements) whose contents or presence are controlled by the
/// current element. See related aria-owns.
///
/// <https://w3c.github.io/aria/#aria-controls>
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Attribute)]
#[attribute(name = "aria-controls")]
pub struct AriaControls(pub Cow<'static, str>);

/// Identifies the element (or elements) that describes the object. See related aria-labelledby
/// and aria-description.
///
/// <https://w3c.github.io/aria/#aria-describedby>
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Attribute)]
#[attribute(name = "aria-describedby")]
pub struct AriaDescribedby(pub Cow<'static, str>);

/// Defines a string value that describes or annotates the current element. See related
/// aria-describedby.
///
/// <https://w3c.github.io/aria/#aria-description>
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Attribute)]
#[attribute(name = "aria-description")]
pub struct AriaDescription(pub Cow<'static, str>);

/// Identifies the element (or elements) that provide additional information related to the
/// object. See related aria-describedby.
///
/// <https://w3c.github.io/aria/#aria-details>
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Attribute)]
#[attribute(name = "aria-details")]
pub struct AriaDetails(pub Cow<'static, str>);

/// Indicates that the element is perceivable but disabled, so it is not editable or otherwise
/// operable. See related aria-hidden and aria-readonly.
///
/// <https://w3c.github.io/aria/#aria-disabled>
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Attribute)]
#[attribute(name = "aria-disabled")]
pub struct AriaDisabled(pub bool);

/// Identifies the element (or elements) that provides an error message for an object. See
/// related aria-invalid and aria-describedby.
///
/// <https://w3c.github.io/aria/#aria-errormessage>
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Attribute)]
#[attribute(name = "aria-errormessage")]
pub struct AriaErrorMessage(pub Cow<'static, str>);

/// Indicates whether a grouping element owned or controlled by this element is expanded or
/// collapsed.
///
/// <https://w3c.github.io/aria/#aria-expanded>
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Attribute)]
#[attribute(name = "aria-expanded")]
pub struct AriaExpanded(pub bool);

/// Identifies the next element (or elements) in an alternate reading order of content which,
/// at the user's discretion, allows assistive technology to override the general default of
/// reading in document source order.
///
/// <https://w3c.github.io/aria/#aria-flowto>
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Attribute)]
#[attribute(name = "aria-flowto")]
pub struct AriaFlowTo(pub Cow<'static, str>);

/// Indicates whether the element is exposed to an accessibility API. See related
/// aria-disabled.
///
/// <https://w3c.github.io/aria/#aria-hidden>
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Attribute)]
#[attribute(name = "aria-hidden")]
pub struct AriaHidden(pub bool);

/// Defines keyboard shortcuts that an author has implemented to activate or give focus to an
/// element.
///
/// <https://w3c.github.io/aria/#aria-keyshortcuts>
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Attribute)]
#[attribute(name = "aria-keyshortcuts")]
pub struct AriaKeyShortcuts(pub Cow<'static, str>);

/// Defines a string value that labels the current element. See related aria-labelledby.
///
/// <https://w3c.github.io/aria/#aria-label>
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Attribute)]
#[attribute(name = "aria-label")]
pub struct AriaLabel(pub Cow<'static, str>);

/// Identifies the element (or elements) that labels the current element. See related
/// aria-label and aria-describedby.
///
/// <https://w3c.github.io/aria/#aria-labelledby>
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Attribute)]
#[attribute(name = "aria-labelledby")]
pub struct AriaLabelledby(pub Cow<'static, str>);

/// Defines the hierarchical level of an element within a structure.
///
/// <https://w3c.github.io/aria/#aria-level>
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Attribute)]
#[attribute(name = "aria-level")]
pub struct AriaLevel(pub u32);

/// Indicates whether an element is modal when displayed.
///
/// <https://w3c.github.io/aria/#aria-modal>
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Attribute)]
#[attribute(name = "aria-modal")]
pub struct AriaModal(pub bool);

/// Indicates whether a text box accepts multiple lines of input or only a single line.
///
/// <https://w3c.github.io/aria/#aria-multiline>
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Attribute)]
#[attribute(name = "aria-multiline")]
pub struct AriaMultiLine(pub bool);

/// Indicates that the user can select more than one item from the current selectable
/// descendants.
///
/// <https://w3c.github.io/aria/#aria-multiselectable>
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Attribute)]
#[attribute(name = "aria-multiselectable")]
pub struct AriaMultiSelectable(pub bool);

/// Identifies an element (or elements) in order to define a visual, functional, or contextual
/// parent/child relationship between DOM elements where the DOM hierarchy cannot be used to
/// represent the relationship. See related aria-controls.
///
/// <https://w3c.github.io/aria/#aria-owns>
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Attribute)]
#[attribute(name = "aria-owns")]
pub struct AriaOwns(pub Cow<'static, str>);

/// Defines a short hint (a word or short phrase) intended to aid the user with data entry when
/// the control has no value. A hint could be a sample value or a brief description of the
/// expected format.
///
/// <https://w3c.github.io/aria/#aria-placeholder>
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Attribute)]
#[attribute(name = "aria-placeholder")]
pub struct AriaPlaceholder(pub Cow<'static, str>);

/// Defines an element's number or position in the current set of listitems or treeitems. Not
/// required if all elements in the set are present in the DOM. See related aria-setsize.
///
/// <https://w3c.github.io/aria/#aria-posinset>
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Attribute)]
#[attribute(name = "aria-posinset")]
pub struct AriaPosInset(pub u32);

/// Indicates that the element is not editable, but is otherwise operable. See related
/// aria-disabled.
///
/// <https://w3c.github.io/aria/#aria-readonly>
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Attribute)]
#[attribute(name = "aria-readonly")]
pub struct AriaReadonly(pub bool);

/// Indicates that user input is required on the element before a form can be submitted.
///
/// <https://w3c.github.io/aria/#aria-required>
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Attribute)]
#[attribute(name = "aria-required")]
pub struct AriaRequired(pub bool);

/// Defines a human-readable, author-localized description for the role of an element.
///
/// <https://w3c.github.io/aria/#aria-roledescription>
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Attribute)]
#[attribute(name = "aria-roledescription")]
pub struct AriaRoleDescription(pub Cow<'static, str>);

/// Defines the total number of rows in a table, grid, or treegrid. See related aria-rowindex.
///
/// <https://w3c.github.io/aria/#aria-rowcount>
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Attribute)]
#[attribute(name = "aria-rowcount")]
pub struct AriaRowCount(pub i32);

/// Defines an element's row index or position with respect to the total number of rows within
/// a table, grid, or treegrid. See related aria-rowindextext, aria-rowcount, and aria-rowspan.
///
/// <https://w3c.github.io/aria/#aria-rowindex>
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Attribute)]
#[attribute(name = "aria-rowindex")]
pub struct AriaRowIndex(pub u32);

/// Defines a human readable text alternative of aria-rowindex. See related aria-colindextext.
///
/// <https://w3c.github.io/aria/#aria-rowindextext>
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Attribute)]
#[attribute(name = "aria-rowindextext")]
pub struct AriaRowIndexText(pub Cow<'static, str>);

/// Defines the number of rows spanned by a cell or gridcell within a table, grid, or treegrid.
/// See related aria-rowindex and aria-colspan.
///
/// <https://w3c.github.io/aria/#aria-rowspan>
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Attribute)]
#[attribute(name = "aria-rowspan")]
pub struct AriaRowSpan(pub u32);

/// Indicates the current "selected" state of various widgets. See related aria-checked and
/// aria-pressed.
///
/// <https://w3c.github.io/aria/#aria-selected>
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Attribute)]
#[attribute(name = "aria-selected")]
pub struct AriaSelected(pub bool);

/// Defines the number of items in the current set of listitems or treeitems. Not required if
/// all elements in the set are present in the DOM. See related aria-posinset.
///
/// <https://w3c.github.io/aria/#aria-setsize>
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Attribute)]
#[attribute(name = "aria-setsize")]
pub struct AriaSetSize(pub i32);

/// Defines the maximum allowed value for a range widget.
///
/// <https://w3c.github.io/aria/#aria-valuemax>
#[derive(Debug, Clone, PartialEq, PartialOrd, Hash, Attribute)]
#[attribute(name = "aria-valuemax")]
pub struct AriaValueMax(pub Number);

/// Defines the minimum allowed value for a range widget.
///
/// <https://w3c.github.io/aria/#aria-valuemin>
#[derive(Debug, Clone, PartialEq, PartialOrd, Hash, Attribute)]
#[attribute(name = "aria-valuemin")]
pub struct AriaValueMin(pub Number);

/// Defines the current value for a range widget. See related aria-valuetext.
///
/// <https://w3c.github.io/aria/#aria-valuenow>
#[derive(Debug, Clone, PartialEq, PartialOrd, Hash, Attribute)]
#[attribute(name = "aria-valuenow")]
pub struct AriaValueNow(pub Number);

/// Defines the human readable text alternative of aria-valuenow for a range widget.
///
/// <https://w3c.github.io/aria/#aria-valuetext>
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Attribute)]
#[attribute(name = "aria-valuetext")]
pub struct AriaValueText(pub Cow<'static, str>);

/// Indicates whether inputting text could trigger display of one or more predictions of the
/// user's intended value for a combobox, searchbox, or textbox and specifies how predictions
/// would be presented if they were made.
///
/// <https://w3c.github.io/aria/#aria-autocomplete>
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Attribute)]
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

/// Indicates the current "checked" state of checkboxes, radio buttons, and other widgets. See
/// related aria-pressed and aria-selected.
///
/// <https://w3c.github.io/aria/#aria-checked>
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Attribute)]
#[attribute(name = "aria-checked")]
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

/// Indicates the element that represents the current item within a container or set of related
/// elements.
///
/// <https://w3c.github.io/aria/#aria-current>
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Attribute)]
#[attribute(name = "aria-current")]
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

/// Indicates the availability and type of interactive popup element, such as menu or dialog,
/// that can be triggered by an element.
///
/// <https://w3c.github.io/aria/#aria-haspopup>
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Attribute)]
#[attribute(name = "aria-haspopup")]
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

/// Indicates the entered value does not conform to the format expected by the application.
/// See related aria-errormessage.
///
/// <https://w3c.github.io/aria/#aria-invalid>
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Attribute)]
#[attribute(name = "aria-invalid")]
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

/// Indicates that an element will be updated, and describes the types of updates the user
/// agents, assistive technologies, and user can expect from the live region.
///
/// <https://w3c.github.io/aria/#aria-live>
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Attribute)]
#[attribute(name = "aria-live")]
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

/// Indicates whether the element's orientation is horizontal, vertical, or unknown/ambiguous.
///
/// <https://w3c.github.io/aria/#aria-orientation>
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Attribute)]
#[attribute(name = "aria-orientation")]
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

/// Indicates the current "pressed" state of toggle buttons. See related aria-checked and
/// aria-selected.
///
/// <https://w3c.github.io/aria/#aria-pressed>
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Attribute)]
#[attribute(name = "aria-pressed")]
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

/// Indicates what notifications the user agent will trigger when the accessibility tree within
/// a live region is modified. See related aria-atomic.
///
/// <https://w3c.github.io/aria/#aria-relevant>
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Attribute)]
#[attribute(name = "aria-relevant")]
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

/// Indicates if items in a table or grid are sorted in ascending or descending order.
///
/// <https://w3c.github.io/aria/#aria-sort>
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Attribute)]
#[attribute(name = "aria-sort")]
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
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
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
