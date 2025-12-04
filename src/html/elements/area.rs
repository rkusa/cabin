use std::borrow::Cow;
use std::fmt::{self};

use cabin_macros::Attribute;

use super::anchor::{Download, Href, Ping, ReferrerPolicy, Rel, RelList, Target};
use super::aria::Aria;
use super::common::Common;
use super::global::Global;
use super::img::Alt;
use crate::attribute::WithAttribute;
use crate::context::Context;
use crate::html::list::SpaceSeparated;
use crate::void_element::VoidElement;

impl Context {
    /// The `area` element represents either a hyperlink with some text and a corresponding area on
    /// an image map, or a dead area on an image map.
    pub fn area(&self) -> VoidElement<'_, marker::Area> {
        VoidElement::new(self, "area")
    }
}

pub mod marker {
    pub struct Area;
}

impl<'v> Area for VoidElement<'v, marker::Area> {}
impl<'v> Common for VoidElement<'v, marker::Area> {}
impl<'v> Global for VoidElement<'v, marker::Area> {}
impl<'v> Aria for VoidElement<'v, marker::Area> {}

/// The `area` element represents either a hyperlink with some text and a corresponding area on an
/// image map, or a dead area on an image map.
pub trait Area: WithAttribute {
    /// Replacement text for use when images are not available.
    fn alt(self, alt: impl Into<Cow<'static, str>>) -> Self {
        self.with_attribute(Alt(alt.into()))
    }

    /// Coordinates for the shape to be created in an image map.
    fn coords(self, coords: impl Into<Cow<'static, str>>) -> Self {
        self.with_attribute(Coords(coords.into()))
    }

    /// The kind of shape to be created in an image map.
    fn shape(self, shape: Shape) -> Self {
        self.with_attribute(shape)
    }

    /// Address of the hyperlink.
    fn href(self, href: impl Into<Cow<'static, str>>) -> Self {
        self.with_attribute(Href(href.into()))
    }

    /// The _browsing context_ the link should be opened in.
    fn target(self, target: impl Into<Cow<'static, str>>) -> Self {
        self.with_attribute(Target(target.into()))
    }

    /// Try to open the link in a new tab.
    fn target_blank(self) -> Self {
        self.with_attribute(Target(Cow::Borrowed("_blank")))
    }

    /// Open the link in the parent browsing context.
    fn target_parent(self) -> Self {
        self.with_attribute(Target(Cow::Borrowed("_parent")))
    }

    /// Open the link in the topmost browsing context.
    fn target_top(self) -> Self {
        self.with_attribute(Target(Cow::Borrowed("_top")))
    }

    /// Treat the linked URL as a download with the specified filename.
    fn download_filename(self, download: impl Into<Cow<'static, str>>) -> Self {
        self.with_attribute(Download(download.into()))
    }

    /// Treat the linked URL as a download and let the browser suggest a filename.
    fn download(self) -> Self {
        self.with_attribute(Download(Cow::Borrowed("")))
    }

    /// A space-separated list of URLs the browser will send POST requests (with the body PING)
    /// when the link is followed (typically used for tracking).
    fn ping(self, ping: impl Into<Cow<'static, str>>) -> Self {
        self.with_attribute(Ping(ping.into()))
    }

    /// Relationship between the location in the document containing the hyperlink and the
    /// destination resource.
    fn rel(self, rel: impl Into<SpaceSeparated<Rel>>) -> Self {
        self.with_attribute(RelList(rel.into()))
    }

    /// How much referrer information to send.
    fn referrer_policy(self, referrer_policy: ReferrerPolicy) -> Self {
        self.with_attribute(referrer_policy)
    }
}

/// Coordinates for the shape to be created in an image map.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Attribute)]
pub struct Coords(pub Cow<'static, str>);

/// Data type of an input element.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Attribute)]
pub enum Shape {
    Circle,
    Default,
    Polygon,
    Rectangle,
}

impl fmt::Display for Shape {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Shape::Circle => "circ",
            Shape::Default => "default",
            Shape::Polygon => "poly",
            Shape::Rectangle => "rect",
        })
    }
}
