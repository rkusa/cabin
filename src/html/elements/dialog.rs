use cabin_macros::Element;

/// A `dialog` element represents a transitory part of an application (e.g. dialog box).
#[derive(Default, Element)]
pub struct Dialog<Ext = ()> {
    pub extension: Ext,

    /// Whether the dialog is showing.
    pub open: bool,
}
