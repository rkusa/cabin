use cabin_macros::Element;

/// The `time` element represents a section with navigation links.
#[derive(Default, Element)]
pub struct Span<Ext = ()> {
    pub extension: Ext,
}
