use std::borrow::Cow;
use std::fmt::{self};

use cabin_macros::Attribute;

use super::anchor::{Download, Href, Ping, ReferrerPolicy, Rel, RelList, Target};
use super::common::Common;
use super::global::Global;
use super::input::Alt;
use crate::html::attributes::{Attributes, WithAttribute};
use crate::html::list::SpaceSeparated;
use crate::html::{Aria, Html};

/// The `area` element represents either a hyperlink with some text and a corresponding area on an
/// image map, or a dead area on an image map.
pub fn area() -> Html<marker::Area, (), ()> {
    Html::new("area", (), ())
}

pub mod marker {
    pub struct Area;
}

impl<A: Attributes, V: 'static> Area for Html<marker::Area, A, V> {}
impl<A: Attributes, V: 'static> Common for Html<marker::Area, A, V> {}
impl<A: Attributes, V: 'static> Global for Html<marker::Area, A, V> {}
impl<A: Attributes, V: 'static> Aria for Html<marker::Area, A, V> {}

/// The `area` element represents either a hyperlink with some text and a corresponding area on an
/// image map, or a dead area on an image map.
pub trait Area: WithAttribute {
    /// Replacement text for use when images are not available.
    fn alt(self, alt: impl Into<Cow<'static, str>>) -> Self::Output<Alt> {
        self.with_attribute(Alt(alt.into()))
    }

    /// Coordinates for the shape to be created in an image map.
    fn coords(self, coords: impl Into<Cow<'static, str>>) -> Self::Output<Coords> {
        self.with_attribute(Coords(coords.into()))
    }

    /// The kind of shape to be created in an image map.
    fn shape(self, shape: Shape) -> Self::Output<Shape> {
        self.with_attribute(shape)
    }

    /// Address of the hyperlink.
    fn href(self, href: impl Into<Cow<'static, str>>) -> Self::Output<Href> {
        self.with_attribute(Href(href.into()))
    }

    /// The _browsing context_ the link should be opened in.
    fn target(self, target: impl Into<Cow<'static, str>>) -> Self::Output<Target> {
        self.with_attribute(Target(target.into()))
    }

    /// Try to open the link in a new tab.
    fn target_blank(self) -> Self::Output<Target> {
        self.with_attribute(Target(Cow::Borrowed("_blank")))
    }

    /// Open the link in the parent browsing context.
    fn target_parent(self) -> Self::Output<Target> {
        self.with_attribute(Target(Cow::Borrowed("_parent")))
    }

    /// Open the link in the topmost browsing context.
    fn target_top(self) -> Self::Output<Target> {
        self.with_attribute(Target(Cow::Borrowed("_top")))
    }

    /// Treat the linked URL as a download with the specified filename.
    fn download_filename(self, download: impl Into<Cow<'static, str>>) -> Self::Output<Download> {
        self.with_attribute(Download(download.into()))
    }

    /// Treat the linked URL as a download and let the browser suggest a filename.
    fn download(self) -> Self::Output<Download> {
        self.with_attribute(Download(Cow::Borrowed("")))
    }

    /// A space-separated list of URLs the browser will send POST requests (with the body PING)
    /// when the link is followed (typically used for tracking).
    fn ping(self, ping: impl Into<Cow<'static, str>>) -> Self::Output<Ping> {
        self.with_attribute(Ping(ping.into()))
    }

    /// Relationship between the location in the document containing the hyperlink and the
    /// destination resource.
    fn rel(self, rel: impl Into<SpaceSeparated<Rel>>) -> Self::Output<RelList> {
        self.with_attribute(RelList(rel.into()))
    }

    /// Appends a [Rel] to the link.
    fn append_rel(mut self, rel: Rel) -> Self::Output<RelList> {
        let rel_list = if let Some(list) = self.get_attribute_mut::<RelList>() {
            RelList(
                match std::mem::replace(&mut list.0, SpaceSeparated::Single(Rel::Alternate)) {
                    SpaceSeparated::Single(existing) => {
                        SpaceSeparated::List([existing, rel].into())
                    }
                    SpaceSeparated::List(mut list) => {
                        list.insert(rel);
                        SpaceSeparated::List(list)
                    }
                },
            )
        } else {
            RelList(SpaceSeparated::Single(rel))
        };
        self.with_attribute(rel_list)
    }

    /// How much referrer information to send.
    fn referrer_policy(self, referrer_policy: ReferrerPolicy) -> Self::Output<ReferrerPolicy> {
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
