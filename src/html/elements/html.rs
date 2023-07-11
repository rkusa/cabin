use cabin_macros::Element;

/// The `html` element represents the root of an HTML document.
#[derive(Default, Element)]
pub struct Html<Ext = ()> {
    pub extension: Ext,
}
