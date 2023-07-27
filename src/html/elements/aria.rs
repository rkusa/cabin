use std::borrow::Cow;
use std::fmt;
use std::hash::{Hash, Hasher};
use std::ops::{Deref, DerefMut};

use cabin_macros::{element, Attribute};

use crate::html::attributes::{Attributes, Pair};

#[element(tag = false)]
pub trait Aria: Attributes {
    // Set the aria role of the element.
    fn role(self, role: Role) -> Pair<Role, Self> {
        self.with(role)
    }

    /// Identifies the currently active element when DOM focus is on a composite widget, combobox,
    /// textbox, group, or application.
    ///
    /// <https://w3c.github.io/aria/#aria-activedescendant>
    fn aria_active_descendant(
        self,
        aria_active_descendant: impl Into<Cow<'static, str>>,
    ) -> Pair<AriaActiveDescendant, Self> {
        self.with(AriaActiveDescendant(aria_active_descendant.into()))
    }

    /// Indicates whether assistive technologies will present all, or only parts of, the changed
    /// region based on the change notifications defined by the aria-relevant attribute.
    ///
    /// <https://w3c.github.io/aria/#aria-atomic>
    fn aria_atomic(self) -> Pair<AriaAtomic, Self> {
        self.with_aria_atomic(true)
    }

    /// Indicates whether assistive technologies will present all, or only parts of, the changed
    /// region based on the change notifications defined by the aria-relevant attribute.
    ///
    /// <https://w3c.github.io/aria/#aria-atomic>
    fn with_aria_atomic(self, aria_atomic: bool) -> Pair<AriaAtomic, Self> {
        self.with(AriaAtomic(aria_atomic))
    }

    /// Indicates whether inputting text could trigger display of one or more predictions of the
    /// user's intended value for a combobox, searchbox, or textbox and specifies how predictions
    /// would be presented if they were made.
    ///
    /// <https://w3c.github.io/aria/#aria-autocomplete>
    fn aria_autocomplete(
        self,
        aria_autocomplete: AutoAutocomplete,
    ) -> Pair<AutoAutocomplete, Self> {
        self.with(aria_autocomplete)
    }

    /// Defines a string value that labels the current element, which is intended to be converted
    /// into Braille. See related aria-label.
    ///
    /// <https://w3c.github.io/aria/#aria-braillelabel>
    fn aria_braille_label(
        self,
        aria_braille_label: impl Into<Cow<'static, str>>,
    ) -> Pair<AriaBrailleLabel, Self> {
        self.with(AriaBrailleLabel(aria_braille_label.into()))
    }

    /// Defines a human-readable, author-localized abbreviated description for the role of an
    /// element, which is intended to be converted into Braille. See related aria-roledescription.
    ///
    /// <https://w3c.github.io/aria/#aria-brailleroledescription>
    fn aria_braille_role_description(
        self,
        aria_braille_role_description: impl Into<Cow<'static, str>>,
    ) -> Pair<AriaBrailleRoleDescription, Self> {
        self.with(AriaBrailleRoleDescription(
            aria_braille_role_description.into(),
        ))
    }

    /// Indicates an element is being modified and that assistive technologies could wait until the
    /// modifications are complete before exposing them to the user.
    ///
    /// <https://w3c.github.io/aria/#aria-busy>
    fn aria_busy(self) -> Pair<AriaBusy, Self> {
        self.with_aria_busy(true)
    }

    /// Indicates an element is being modified and that assistive technologies could wait until the
    /// modifications are complete before exposing them to the user.
    ///
    /// <https://w3c.github.io/aria/#aria-busy>
    fn with_aria_busy(self, aria_busy: bool) -> Pair<AriaBusy, Self> {
        self.with(AriaBusy(aria_busy))
    }

    /// Indicates the current "checked" state of checkboxes, radio buttons, and other widgets. See
    /// related aria-pressed and aria-selected.
    ///
    /// <https://w3c.github.io/aria/#aria-checked>
    fn aria_checked(self, aria_checked: AriaChecked) -> Pair<AriaChecked, Self> {
        self.with(aria_checked)
    }

    /// Defines the total number of columns in a table, grid, or treegrid. See related
    /// aria-colindex.
    ///
    /// <https://w3c.github.io/aria/#aria-colcount>
    fn aria_col_count(self, aria_col_count: impl Into<i32>) -> Pair<AriaColCount, Self> {
        self.with(AriaColCount(aria_col_count.into()))
    }

    /// Defines an element's column index or position with respect to the total number of columns
    /// within a table, grid, or treegrid. See related aria-colindextext, aria-colcount, and
    /// aria-colspan.
    ///
    /// <https://w3c.github.io/aria/#aria-colindex>
    fn aria_col_index(self, aria_col_index: impl Into<u32>) -> Pair<AriaColIndex, Self> {
        self.with(AriaColIndex(aria_col_index.into()))
    }

    /// Defines a human readable text alternative of aria-colindex. See related aria-rowindextext.
    ///
    /// <https://w3c.github.io/aria/#aria-colindextext>
    fn aria_colindextext(
        self,
        aria_colindextext: impl Into<Cow<'static, str>>,
    ) -> Pair<AriaColindextext, Self> {
        self.with(AriaColindextext(aria_colindextext.into()))
    }

    /// Defines the number of columns spanned by a cell or gridcell within a table, grid, or
    /// treegrid. See related aria-colindex and aria-rowspan.
    ///
    /// <https://w3c.github.io/aria/#aria-colspan>
    fn aria_col_span(self, aria_col_span: impl Into<u32>) -> Pair<AriaColSpan, Self> {
        self.with(AriaColSpan(aria_col_span.into()))
    }

    /// Identifies the element (or elements) whose contents or presence are controlled by the
    /// current element. See related aria-owns.
    ///
    /// <https://w3c.github.io/aria/#aria-controls>
    fn aria_controls(
        self,
        aria_controls: impl Into<Cow<'static, str>>,
    ) -> Pair<AriaControls, Self> {
        self.with(AriaControls(aria_controls.into()))
    }

    /// Indicates the element that represents the current item within a container or set of related
    /// elements.
    ///
    /// <https://w3c.github.io/aria/#aria-current>
    fn aria_current(self, aria_current: AriaCurrent) -> Pair<AriaCurrent, Self> {
        self.with(aria_current)
    }

    /// Identifies the element (or elements) that describes the object. See related aria-labelledby
    /// and aria-description.
    ///
    /// <https://w3c.github.io/aria/#aria-describedby>
    fn aria_described_by(
        self,
        aria_describedby: impl Into<Cow<'static, str>>,
    ) -> Pair<AriaDescribedBy, Self> {
        self.with(AriaDescribedBy(aria_describedby.into()))
    }

    /// Defines a string value that describes or annotates the current element. See related
    /// aria-describedby.
    ///
    /// <https://w3c.github.io/aria/#aria-description>
    fn aria_description(
        self,
        aria_description: impl Into<Cow<'static, str>>,
    ) -> Pair<AriaDescription, Self> {
        self.with(AriaDescription(aria_description.into()))
    }

    /// Identifies the element (or elements) that provide additional information related to the
    /// object. See related aria-describedby.
    ///
    /// <https://w3c.github.io/aria/#aria-details>
    fn aria_details(self, aria_details: impl Into<Cow<'static, str>>) -> Pair<AriaDetails, Self> {
        self.with(AriaDetails(aria_details.into()))
    }

    /// Indicates that the element is perceivable but disabled, so it is not editable or otherwise
    /// operable. See related aria-hidden and aria-readonly.
    ///
    /// <https://w3c.github.io/aria/#aria-disabled>
    fn aria_disabled(self) -> Pair<AriaDisabled, Self> {
        self.with_aria_disabled(true)
    }

    /// Indicates that the element is perceivable but disabled, so it is not editable or otherwise
    /// operable. See related aria-hidden and aria-readonly.
    ///
    /// <https://w3c.github.io/aria/#aria-disabled>
    fn with_aria_disabled(self, aria_disabled: bool) -> Pair<AriaDisabled, Self> {
        self.with(AriaDisabled(aria_disabled))
    }

    /// Identifies the element (or elements) that provides an error message for an object. See
    /// related aria-invalid and aria-describedby.
    ///
    /// <https://w3c.github.io/aria/#aria-errormessage>
    fn aria_error_message(
        self,
        aria_error_message: impl Into<Cow<'static, str>>,
    ) -> Pair<AriaErrorMessage, Self> {
        self.with(AriaErrorMessage(aria_error_message.into()))
    }

    /// Indicates whether a grouping element owned or controlled by this element is expanded or
    /// collapsed.
    ///
    /// <https://w3c.github.io/aria/#aria-expanded>
    fn aria_expanded(self) -> Pair<AriaExpanded, Self> {
        self.with_aria_expanded(true)
    }

    /// Indicates whether a grouping element owned or controlled by this element is expanded or
    /// collapsed.
    ///
    /// <https://w3c.github.io/aria/#aria-expanded>
    fn with_aria_expanded(self, aria_expanded: bool) -> Pair<AriaExpanded, Self> {
        self.with(AriaExpanded(aria_expanded))
    }

    /// Identifies the next element (or elements) in an alternate reading order of content which,
    /// at the user's discretion, allows assistive technology to override the general default of
    /// reading in document source order.
    ///
    /// <https://w3c.github.io/aria/#aria-flowto>
    fn aria_flow_to(self, aria_flow_to: impl Into<Cow<'static, str>>) -> Pair<AriaFlowTo, Self> {
        self.with(AriaFlowTo(aria_flow_to.into()))
    }

    /// Indicates the availability and type of interactive popup element, such as menu or dialog,
    /// that can be triggered by an element.
    ///
    /// <https://w3c.github.io/aria/#aria-haspopup>
    fn aria_haspopup(self, aria_haspopup: AriaHasPopup) -> Pair<AriaHasPopup, Self> {
        self.with(aria_haspopup)
    }

    /// Indicates whether the element is exposed to an accessibility API. See related
    /// aria-disabled.
    ///
    /// <https://w3c.github.io/aria/#aria-hidden>
    fn aria_hidden(self, aria_hidden: bool) -> Pair<AriaHidden, Self> {
        self.with(AriaHidden(aria_hidden))
    }

    /// Indicates the entered value does not conform to the format expected by the application.
    /// See related aria-errormessage.
    ///
    /// <https://w3c.github.io/aria/#aria-invalid>
    fn aria_invalid(self, aria_invalid: AriaInvalid) -> Pair<AriaInvalid, Self> {
        self.with(aria_invalid)
    }

    /// Defines keyboard shortcuts that an author has implemented to activate or give focus to an
    /// element.
    ///
    /// <https://w3c.github.io/aria/#aria-keyshortcuts>
    fn aria_key_shortcuts(
        self,
        aria_key_shortcuts: impl Into<Cow<'static, str>>,
    ) -> Pair<AriaKeyShortcuts, Self> {
        self.with(AriaKeyShortcuts(aria_key_shortcuts.into()))
    }

    /// Defines a string value that labels the current element. See related aria-labelledby.
    ///
    /// <https://w3c.github.io/aria/#aria-label>
    fn aria_label(self, aria_label: impl Into<Cow<'static, str>>) -> Pair<AriaLabel, Self> {
        self.with(AriaLabel(aria_label.into()))
    }

    /// Identifies the element (or elements) that labels the current element. See related
    /// aria-label and aria-describedby.
    ///
    /// <https://w3c.github.io/aria/#aria-labelledby>
    fn aria_labelledby(
        self,
        aria_labelledby: impl Into<Cow<'static, str>>,
    ) -> Pair<AriaLabelledby, Self> {
        self.with(AriaLabelledby(aria_labelledby.into()))
    }

    /// Defines the hierarchical level of an element within a structure.
    ///
    /// <https://w3c.github.io/aria/#aria-level>
    fn aria_level(self, aria_level: impl Into<u32>) -> Pair<AriaLevel, Self> {
        self.with(AriaLevel(aria_level.into()))
    }

    /// Indicates that an element will be updated, and describes the types of updates the user
    /// agents, assistive technologies, and user can expect from the live region.
    ///
    /// <https://w3c.github.io/aria/#aria-live>
    fn aria_live(self, aria_live: AriaLive) -> Pair<AriaLive, Self> {
        self.with(aria_live)
    }

    /// Indicates whether an element is modal when displayed.
    ///
    /// <https://w3c.github.io/aria/#aria-modal>
    fn aria_modal(self) -> Pair<AriaModal, Self> {
        self.with_aria_modal(true)
    }

    /// Indicates whether an element is modal when displayed.
    ///
    /// <https://w3c.github.io/aria/#aria-modal>
    fn with_aria_modal(self, aria_modal: bool) -> Pair<AriaModal, Self> {
        self.with(AriaModal(aria_modal))
    }

    /// Indicates whether a text box accepts multiple lines of input or only a single line.
    ///
    /// <https://w3c.github.io/aria/#aria-multiline>
    fn aria_multi_line(self) -> Pair<AriaMultiLine, Self> {
        self.with_aria_multi_line(true)
    }

    /// Indicates whether a text box accepts multiple lines of input or only a single line.
    ///
    /// <https://w3c.github.io/aria/#aria-multiline>
    fn with_aria_multi_line(self, aria_multi_line: bool) -> Pair<AriaMultiLine, Self> {
        self.with(AriaMultiLine(aria_multi_line))
    }

    /// Indicates that the user can select more than one item from the current selectable
    /// descendants.
    ///
    /// <https://w3c.github.io/aria/#aria-multiselectable>
    fn aria_multi_selectable(self) -> Pair<AriaMultiSelectable, Self> {
        self.with_aria_multi_selectable(true)
    }

    /// Indicates that the user can select more than one item from the current selectable
    /// descendants.
    ///
    /// <https://w3c.github.io/aria/#aria-multiselectable>
    fn with_aria_multi_selectable(
        self,
        aria_multi_selectable: bool,
    ) -> Pair<AriaMultiSelectable, Self> {
        self.with(AriaMultiSelectable(aria_multi_selectable))
    }

    /// Indicates whether the element's orientation is horizontal, vertical, or unknown/ambiguous.
    ///
    /// <https://w3c.github.io/aria/#aria-orientation>
    fn aria_orientation(self, aria_orientation: AriaOrientation) -> Pair<AriaOrientation, Self> {
        self.with(aria_orientation)
    }

    /// Identifies an element (or elements) in order to define a visual, functional, or contextual
    /// parent/child relationship between DOM elements where the DOM hierarchy cannot be used to
    /// represent the relationship. See related aria-controls.
    ///
    /// <https://w3c.github.io/aria/#aria-owns>
    fn aria_owns(self, aria_owns: impl Into<Cow<'static, str>>) -> Pair<AriaOwns, Self> {
        self.with(AriaOwns(aria_owns.into()))
    }

    /// Defines a short hint (a word or short phrase) intended to aid the user with data entry when
    /// the control has no value. A hint could be a sample value or a brief description of the
    /// expected format.
    ///
    /// <https://w3c.github.io/aria/#aria-placeholder>
    fn aria_placeholder(
        self,
        aria_placeholder: impl Into<Cow<'static, str>>,
    ) -> Pair<AriaPlaceholder, Self> {
        self.with(AriaPlaceholder(aria_placeholder.into()))
    }

    /// Defines an element's number or position in the current set of listitems or treeitems. Not
    /// required if all elements in the set are present in the DOM. See related aria-setsize.
    ///
    /// <https://w3c.github.io/aria/#aria-posinset>
    fn aria_pos_inset(self, aria_pos_inset: impl Into<u32>) -> Pair<AriaPosInset, Self> {
        self.with(AriaPosInset(aria_pos_inset.into()))
    }

    /// Indicates the current "pressed" state of toggle buttons. See related aria-checked and
    /// aria-selected.
    ///
    /// <https://w3c.github.io/aria/#aria-pressed>
    fn aria_pressed(self, aria_pressed: AriaPressed) -> Pair<AriaPressed, Self> {
        self.with(aria_pressed)
    }

    /// Indicates that the element is not editable, but is otherwise operable. See related
    /// aria-disabled.
    ///
    /// <https://w3c.github.io/aria/#aria-readonly>
    fn aria_readonly(self) -> Pair<AriaReadonly, Self> {
        self.with_aria_readonly(true)
    }

    /// Indicates that the element is not editable, but is otherwise operable. See related
    /// aria-disabled.
    ///
    /// <https://w3c.github.io/aria/#aria-readonly>
    fn with_aria_readonly(self, aria_readonly: bool) -> Pair<AriaReadonly, Self> {
        self.with(AriaReadonly(aria_readonly))
    }

    /// Indicates what notifications the user agent will trigger when the accessibility tree within
    /// a live region is modified. See related aria-atomic.
    ///
    /// <https://w3c.github.io/aria/#aria-relevant>
    fn aria_relevant(self, aria_relevant: AriaRelevant) -> Pair<AriaRelevant, Self> {
        self.with(aria_relevant)
    }

    /// Indicates that user input is required on the element before a form can be submitted.
    ///
    /// <https://w3c.github.io/aria/#aria-required>
    fn aria_required(self) -> Pair<AriaRequired, Self> {
        self.with_aria_required(true)
    }

    /// Indicates that user input is required on the element before a form can be submitted.
    ///
    /// <https://w3c.github.io/aria/#aria-required>
    fn with_aria_required(self, aria_required: bool) -> Pair<AriaRequired, Self> {
        self.with(AriaRequired(aria_required))
    }

    /// Defines a human-readable, author-localized description for the role of an element.
    ///
    /// <https://w3c.github.io/aria/#aria-roledescription>
    fn aria_role_description(
        self,
        aria_role_description: impl Into<Cow<'static, str>>,
    ) -> Pair<AriaRoleDescription, Self> {
        self.with(AriaRoleDescription(aria_role_description.into()))
    }

    /// Defines the total number of rows in a table, grid, or treegrid. See related aria-rowindex.
    ///
    /// <https://w3c.github.io/aria/#aria-rowcount>
    fn aria_row_count(self, aria_row_count: impl Into<i32>) -> Pair<AriaRowCount, Self> {
        self.with(AriaRowCount(aria_row_count.into()))
    }

    /// Defines an element's row index or position with respect to the total number of rows within
    /// a table, grid, or treegrid. See related aria-rowindextext, aria-rowcount, and aria-rowspan.
    ///
    /// <https://w3c.github.io/aria/#aria-rowindex>
    fn aria_row_index(self, aria_row_index: impl Into<u32>) -> Pair<AriaRowIndex, Self> {
        self.with(AriaRowIndex(aria_row_index.into()))
    }

    /// Defines a human readable text alternative of aria-rowindex. See related aria-colindextext.
    ///
    /// <https://w3c.github.io/aria/#aria-rowindextext>
    fn aria_row_index_text(
        self,
        aria_row_index_text: impl Into<Cow<'static, str>>,
    ) -> Pair<AriaRowIndexText, Self> {
        self.with(AriaRowIndexText(aria_row_index_text.into()))
    }

    /// Defines the number of rows spanned by a cell or gridcell within a table, grid, or treegrid.
    /// See related aria-rowindex and aria-colspan.
    ///
    /// <https://w3c.github.io/aria/#aria-rowspan>
    fn aria_row_span(self, aria_row_span: impl Into<u32>) -> Pair<AriaRowSpan, Self> {
        self.with(AriaRowSpan(aria_row_span.into()))
    }

    /// Indicates the current "selected" state of various widgets. See related aria-checked and
    /// aria-pressed.
    ///
    /// <https://w3c.github.io/aria/#aria-selected>
    fn aria_selected(self) -> Pair<AriaSelected, Self> {
        self.with_aria_selected(true)
    }

    /// Indicates the current "selected" state of various widgets. See related aria-checked and
    /// aria-pressed.
    ///
    /// <https://w3c.github.io/aria/#aria-selected>
    fn with_aria_selected(self, aria_selected: bool) -> Pair<AriaSelected, Self> {
        self.with(AriaSelected(aria_selected))
    }

    /// Defines the number of items in the current set of listitems or treeitems. Not required if
    /// all elements in the set are present in the DOM. See related aria-posinset.
    ///
    /// <https://w3c.github.io/aria/#aria-setsize>
    fn aria_set_size(self, aria_set_size: impl Into<i32>) -> Pair<AriaSetSize, Self> {
        self.with(AriaSetSize(aria_set_size.into()))
    }

    /// Indicates if items in a table or grid are sorted in ascending or descending order.
    ///
    /// <https://w3c.github.io/aria/#aria-sort>
    fn aria_sort(self, aria_sort: AriaSort) -> Pair<AriaSort, Self> {
        self.with(aria_sort)
    }

    /// Defines the maximum allowed value for a range widget.
    ///
    /// <https://w3c.github.io/aria/#aria-valuemax>
    fn aria_value_max(self, aria_value_max: impl Into<Number>) -> Pair<AriaValueMax, Self> {
        self.with(AriaValueMax(aria_value_max.into()))
    }

    /// Defines the minimum allowed value for a range widget.
    ///
    /// <https://w3c.github.io/aria/#aria-valuemin>
    fn aria_value_min(self, aria_value_min: impl Into<Number>) -> Pair<AriaValueMin, Self> {
        self.with(AriaValueMin(aria_value_min.into()))
    }

    /// Defines the current value for a range widget. See related aria-valuetext.
    ///
    /// <https://w3c.github.io/aria/#aria-valuenow>
    fn aria_value_now(self, aria_value_now: impl Into<Number>) -> Pair<AriaValueNow, Self> {
        self.with(AriaValueNow(aria_value_now.into()))
    }

    /// Defines the human readable text alternative of aria-valuenow for a range widget.
    ///
    /// <https://w3c.github.io/aria/#aria-valuetext>
    fn aria_value_text(
        self,
        aria_value_text: impl Into<Cow<'static, str>>,
    ) -> Pair<AriaValueText, Self> {
        self.with(AriaValueText(aria_value_text.into()))
    }
}

/// The aria role of the element.
///
/// <https://w3c.github.io/aria/#role_definitions>
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Attribute)]
pub enum Role {
    /// A type of live region with important, and usually time-sensitive, information. See related
    /// [Role::AlertDialog] and [Role::Status].
    Alert,

    /// A type of dialog that contains an alert message, where initial focus goes to an element
    /// within the dialog. See related [Role::Alert] and [Role::Dialog].
    AlertDialog,

    /// A structure containing one or more focusable elements requiring user input, such as keyboard
    /// or gesture events, that do not follow a standard interaction pattern supported by a [Role::Widget]
    /// role.
    Application,

    /// A section of a page that consists of a composition that forms an independent part of a
    /// document, page, or site.
    Article,

    /// A section containing [Role::AssociationListItemKey] and [Role::AssociationListItemValue] elements.
    AssociationList,

    /// A single key item in an association list.
    AssociationListItemKey,

    /// A single value item in an association list.
    AssociationListItemValue,

    /// A landmark that contains mostly site-oriented content, rather than page-specific content.
    Banner,

    /// A section of content that is quoted from another source.
    Blockquote,

    /// An input that allows for user-triggered actions when clicked or pressed. See related [Role::Link].
    Button,

    /// Visible content that names, or describes a [Role::Group], [Role::Figure], [Role::Table], [Role::Grid], [Role::RadioGroup],
    /// or [Role::TreeGrid].
    Caption,

    /// A cell in a tabular container. See related [Role::GridCell].
    Cell,

    /// A checkable input that has three possible values: true, false, or mixed.
    Checkbox,

    /// A section whose content represents a fragment of computer code.
    Code,

    /// A cell containing header information for a column.
    ColumnHeader,

    /// An input that controls another element, such as a [Role::Listbox] or [Role::Grid], that can dynamically
    /// pop up to help the user set the value of the [Role::Input].
    Combobox,

    /// A form of [Role::Widget] that performs an action but does not receive input data.
    Command,

    /// A comment contains content expressing reaction to other content.
    Comment,

    /// A [Role::Landmark] that is designed to be complementary to the main content that it is a sibling
    /// to, or a direct descendant of. The contents of a complementary landmark would be expected to
    /// remain meaningful if it were to be separated from the main content it is relevant to.
    Complementary,

    /// A [Role::Widget] that can contain navigable descendants or owned children.
    Composite,

    /// A [Role::Landmark] that contains information about the parent document.
    ContentInfo,

    /// A definition of a term or concept. See related [Role::Term].
    Definition,

    /// A deletion represents content that is marked as removed, content that is being suggested for
    /// removal, or content that is no longer relevant in the context of its accompanying content.
    /// See related [Role::Insertion].
    Deletion,

    /// A dialog is a descendant window of the primary window of a web application. For HTML pages,
    /// the primary application window is the entire web document, i.e., the body element.
    Dialog,

    /// An element containing content that assistive technology users might want to browse in a
    /// reading mode.
    Document,

    /// One or more emphasized characters. See related [Role::Strong].
    Emphasis,

    /// A scrollable [Role::List] of [Role::Article]s where scrolling might cause [Role::Article]s to be added to or
    /// removed from either end of the list.
    Feed,

    /// A perceivable [Role::Section] of content that typically contains a graphical document, images,
    /// media player, code snippets, or example text. The parts of a figure MAY be user-navigable.
    Figure,

    /// A [Role::Landmark] region that contains a collection of items and objects that, as a whole,
    /// combine to create a form. See related [Role::Search].
    Form,

    /// A nameless container element that has no semantic meaning on its own.
    Generic,

    /// A composite [Role::Widget] containing a collection of one or more rows with one or more cells where
    /// some or all cells in the grid are focusable by using methods of two-dimensional navigation,
    /// such as directional arrow keys.
    Grid,

    /// A [Role::Cell] in a [Role::Grid] or [Role::TreeGrid].
    GridCell,

    /// A set of user interface objects that is not intended to be included in a page summary or
    /// table of contents by assistive technologies.
    Group,

    /// A heading for a section of the page.
    Heading,

    /// A container for a collection of elements that form an image. See synonym [Role::Img].
    Image,

    /// A container for a collection of elements that form an image. See synonym [Role::Image].
    Img,

    /// A generic type of [Role::Widget] that allows user input.
    Input,

    /// An insertion contains content that is marked as added or content that is being suggested for
    /// addition. See related [Role::Deletion].
    Insertion,

    /// A perceivable [Role::Section] containing content that is relevant to a specific, author-specified
    /// purpose and sufficiently important that users will likely want to be able to navigate to the
    /// section easily and to have it listed in a summary of the page. Such a page summary could be
    /// generated dynamically by a user agent or assistive technology.
    Landmark,

    /// An interactive reference to an internal or external resource that, when activated, causes
    /// the user agent to navigate to that resource. See related [Role::Button].
    Link,

    /// A [Role::Section] containing [Role::ListItem] elements. See related [Role::Listbox].
    List,

    /// A [Role::Widget] that allows the user to select one or more items from a list of choices. See
    /// related [Role::Combobox] and list.
    Listbox,

    /// A single item in a list or directory.
    ListItem,

    /// A type of live region where new information is added in meaningful order and old information
    /// can disappear. See related [Role::Marquee].
    Log,

    /// A [Role::Landmark] containing the main content of a document.
    Main,

    /// Content which is marked or highlighted for reference or notation purposes, due to the
    /// content's relevance in the enclosing context.
    Mark,

    /// A type of live region where non-essential information changes frequently. See related [Role::Log].
    Marquee,

    /// Content that represents a mathematical expression.
    Math,

    /// A type of [Role::Widget] that offers a list of choices to the user.
    Menu,

    /// A presentation of [Role::Menu] that usually remains visible and is usually presented horizontally.
    Menubar,

    /// An option in a set of choices contained by a [Role::Menu] or [Role::Menubar].
    MenuItem,

    /// A [Role::MenuItem] with a checkable state whose possible values are true, false, or mixed.
    MenuItemCheckbox,

    /// A checkable [Role::MenuItem] in a set of elements with the same role, only one of which can be
    /// checked at a time.
    MenuItemRadio,

    /// An element that represents a scalar measurement within a known range, or a fractional value.
    /// See related [Role::Progressbar].
    Meter,

    /// A [Role::Landmark] containing a collection of navigational elements (usually links) for navigating
    /// the document or related documents.
    Navigation,

    /// An element whose implicit native role semantics will not be mapped to the accessibility API.
    /// See synonym [Role::Presentation].
    None,

    /// A [Role::Section] whose content represents additional information or parenthetical context to the
    /// primary content it supplements.
    Note,

    /// An item in a [Role::Listbox].
    Option,

    /// A paragraph of content.
    Paragraph,

    /// An element whose implicit native role semantics will not be mapped to the accessibility API.
    /// See synonym [Role::None].
    Presentation,

    /// An element that displays the progress status for tasks that take a long time.
    Progressbar,

    /// A checkable input in a group of elements with the same role, only one of which can be
    /// checked at a time.
    Radio,

    /// A group of [Role::Radio] buttons.
    RadioGroup,

    /// An element representing a range of values.
    Range,

    /// A [Role::Landmark] containing content that is relevant to a specific, author-specified purpose and
    /// sufficiently important that users will likely want to be able to navigate to the section
    /// easily and to have it listed in a summary of the page. Such a page summary could be
    /// generated dynamically by a user agent or assistive technology.
    Region,

    /// The base role from which all other roles inherit.
    Roletype,

    /// A row of cells in a tabular container.
    Row,

    /// A structure containing one or more row elements in a tabular container.
    RowGroup,

    /// A cell containing header information for a row.
    RowHeader,

    /// A graphical object that controls the scrolling of content within a viewing area, regardless
    /// of whether the content is fully displayed within the viewing area.
    Scrollbar,

    /// A [Role::Landmark] region that contains a collection of items and objects that, as a whole,
    /// combine to create a search facility. See related [Role::Form] and [Role::Searchbox].
    Search,

    /// A type of textbox intended for specifying search criteria. See related [Role::Textbox] and
    /// [Role::Search].
    Searchbox,

    /// A renderable structural containment unit on a page.
    Section,

    /// A structure that labels or summarizes the topic of its related section.
    SectionHead,

    /// A form [Role::Widget] that allows the user to make selections from a set of choices.
    Select,

    /// A divider that separates and distinguishes sections of content or groups of [Role::MenuItem]s.
    Separator,

    /// An input where the user selects a value from within a given range.
    Slider,

    /// A form of [Role::Range] that expects the user to select from among discrete choices.
    SpinButton,

    /// A type of live region whose content is advisory information for the user but is not
    /// important enough to justify an [Role::Alert], often but not necessarily presented as a status bar.
    Status,

    /// Content that is important, serious, or urgent. See related [Role::Emphasis].
    Strong,

    /// A document structural element.
    Structure,

    /// One or more subscripted characters. See related [Role::Superscript].
    Subscript,

    /// A single proposed change to content.
    Suggestion,

    /// One or more superscripted characters. See related [Role::Subscript].
    Superscript,

    /// A type of checkbox that represents on/off values, as opposed to checked/unchecked values.
    /// See related [Role::Checkbox].
    Switch,

    /// A grouping label providing a mechanism for selecting the tab content that is to be rendered
    /// to the user.
    Tab,

    /// A [Role::Section] containing data arranged in rows and columns. See related [Role::Grid].
    Table,

    /// A list of [Role::Tab] elements, which are references to [Role::TabPanel] elements.
    TabList,

    /// A container for the resources associated with a [Role::Tab], where each tab is contained in a
    /// [Role::TabList].
    TabPanel,

    /// A word or phrase with an optional corresponding definition. See related [Role::Definition].
    Term,

    /// A type of input that allows free-form text as its value.
    Textbox,

    /// An element that represents a specific point in time.
    Time,

    /// A type of live region containing a numerical counter which indicates an amount of elapsed
    /// time from a start point, or the time remaining until an end point.
    Timer,

    /// A collection of commonly used function buttons or controls represented in compact visual
    /// form.
    Toolbar,

    /// A contextual popup that displays a description for an element.
    Tooltip,

    /// A [Role::Widget] that allows the user to select one or more items from a hierarchically organized
    /// collection.
    Tree,

    /// A grid whose rows can be expanded and collapsed in the same manner as for a [Role::Tree].
    TreeGrid,

    /// An item in a [Role::Tree].
    TreeItem,

    /// An interactive component of a graphical user interface (GUI).
    Widget,

    /// A browser or application window.
    Window,
}

impl fmt::Display for Role {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Role::Alert => f.write_str("alert"),
            Role::AlertDialog => f.write_str("alertdialog"),
            Role::Application => f.write_str("application"),
            Role::Article => f.write_str("article"),
            Role::AssociationList => f.write_str("associationlist"),
            Role::AssociationListItemKey => f.write_str("associationlistitemkey"),
            Role::AssociationListItemValue => f.write_str("associationlistitemvalue"),
            Role::Banner => f.write_str("banner"),
            Role::Blockquote => f.write_str("blockquote"),
            Role::Button => f.write_str("button"),
            Role::Caption => f.write_str("caption"),
            Role::Cell => f.write_str("cell"),
            Role::Checkbox => f.write_str("checkbox"),
            Role::Code => f.write_str("code"),
            Role::ColumnHeader => f.write_str("columnheader"),
            Role::Combobox => f.write_str("combobox"),
            Role::Command => f.write_str("command"),
            Role::Comment => f.write_str("comment"),
            Role::Complementary => f.write_str("complementary"),
            Role::Composite => f.write_str("composite"),
            Role::ContentInfo => f.write_str("contentinfo"),
            Role::Definition => f.write_str("definition"),
            Role::Deletion => f.write_str("deletion"),
            Role::Dialog => f.write_str("dialog"),
            Role::Document => f.write_str("document"),
            Role::Emphasis => f.write_str("emphasis"),
            Role::Feed => f.write_str("feed"),
            Role::Figure => f.write_str("figure"),
            Role::Form => f.write_str("form"),
            Role::Generic => f.write_str("generic"),
            Role::Grid => f.write_str("grid"),
            Role::GridCell => f.write_str("gridcell"),
            Role::Group => f.write_str("group"),
            Role::Heading => f.write_str("heading"),
            Role::Image => f.write_str("image"),
            Role::Img => f.write_str("img"),
            Role::Input => f.write_str("input"),
            Role::Insertion => f.write_str("insertion"),
            Role::Landmark => f.write_str("landmark"),
            Role::Link => f.write_str("link"),
            Role::List => f.write_str("list"),
            Role::Listbox => f.write_str("listbox"),
            Role::ListItem => f.write_str("listitem"),
            Role::Log => f.write_str("log"),
            Role::Main => f.write_str("main"),
            Role::Mark => f.write_str("mark"),
            Role::Marquee => f.write_str("marquee"),
            Role::Math => f.write_str("math"),
            Role::Menu => f.write_str("menu"),
            Role::Menubar => f.write_str("menubar"),
            Role::MenuItem => f.write_str("menuitem"),
            Role::MenuItemCheckbox => f.write_str("menuitemcheckbox"),
            Role::MenuItemRadio => f.write_str("menuitemradio"),
            Role::Meter => f.write_str("meter"),
            Role::Navigation => f.write_str("navigation"),
            Role::None => f.write_str("none"),
            Role::Note => f.write_str("note"),
            Role::Option => f.write_str("option"),
            Role::Paragraph => f.write_str("paragraph"),
            Role::Presentation => f.write_str("presentation"),
            Role::Progressbar => f.write_str("progressbar"),
            Role::Radio => f.write_str("radio"),
            Role::RadioGroup => f.write_str("radiogroup"),
            Role::Range => f.write_str("range"),
            Role::Region => f.write_str("region"),
            Role::Roletype => f.write_str("roletype"),
            Role::Row => f.write_str("row"),
            Role::RowGroup => f.write_str("rowgroup"),
            Role::RowHeader => f.write_str("rowheader"),
            Role::Scrollbar => f.write_str("scrollbar"),
            Role::Search => f.write_str("search"),
            Role::Searchbox => f.write_str("searchbox"),
            Role::Section => f.write_str("section"),
            Role::SectionHead => f.write_str("sectionhead"),
            Role::Select => f.write_str("select"),
            Role::Separator => f.write_str("separator"),
            Role::Slider => f.write_str("slider"),
            Role::SpinButton => f.write_str("spinbutton"),
            Role::Status => f.write_str("status"),
            Role::Strong => f.write_str("strong"),
            Role::Structure => f.write_str("structure"),
            Role::Subscript => f.write_str("subscript"),
            Role::Suggestion => f.write_str("suggestion"),
            Role::Superscript => f.write_str("superscript"),
            Role::Switch => f.write_str("switch"),
            Role::Tab => f.write_str("tab"),
            Role::Table => f.write_str("table"),
            Role::TabList => f.write_str("tablist"),
            Role::TabPanel => f.write_str("tabpanel"),
            Role::Term => f.write_str("term"),
            Role::Textbox => f.write_str("textbox"),
            Role::Time => f.write_str("time"),
            Role::Timer => f.write_str("timer"),
            Role::Toolbar => f.write_str("toolbar"),
            Role::Tooltip => f.write_str("tooltip"),
            Role::Tree => f.write_str("tree"),
            Role::TreeGrid => f.write_str("treegrid"),
            Role::TreeItem => f.write_str("treeitem"),
            Role::Widget => f.write_str("widget"),
            Role::Window => f.write_str("window"),
        }
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
pub struct AriaDescribedBy(pub Cow<'static, str>);

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
            Self::Inline => f.write_str("inline"),
            Self::List => f.write_str("list"),
            Self::Both => f.write_str("both"),
            Self::None => f.write_str("none"),
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
            Self::True => f.write_str("true"),
            Self::False => f.write_str("false"),
            Self::Mixed => f.write_str("mixed"),
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
            Self::Page => f.write_str("page"),
            Self::Step => f.write_str("step"),
            Self::Location => f.write_str("location"),
            Self::Date => f.write_str("date"),
            Self::Time => f.write_str("time"),
            Self::True => f.write_str("true"),
            Self::False => f.write_str("false"),
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
            Self::False => f.write_str("false"),
            Self::True => f.write_str("true"),
            Self::Menu => f.write_str("menu"),
            Self::Listbox => f.write_str("listbox"),
            Self::Tree => f.write_str("tree"),
            Self::Grid => f.write_str("grid"),
            Self::Dialog => f.write_str("dialog"),
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
            Self::Grammar => f.write_str("grammar"),
            Self::False => f.write_str("false"),
            Self::Spelling => f.write_str("spelling"),
            Self::True => f.write_str("true"),
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
            Self::Assertive => f.write_str("assertive"),
            Self::Off => f.write_str("off"),
            Self::Polite => f.write_str("polite"),
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
            Self::Horizontal => f.write_str("horizontal"),
            Self::Vertical => f.write_str("vertical"),
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
            Self::True => f.write_str("true"),
            Self::False => f.write_str("false"),
            Self::Mixed => f.write_str("mixed"),
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
            Self::Additions => f.write_str("additions"),
            Self::AdditionsText => f.write_str("additions text"),
            Self::All => f.write_str("all"),
            Self::Removals => f.write_str("removals"),
            Self::Text => f.write_str("text"),
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
            Self::Ascending => f.write_str("ascending"),
            Self::Descending => f.write_str("descending"),
            Self::Other => f.write_str("other"),
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
