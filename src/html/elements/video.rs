use std::borrow::Cow;

use cabin_macros::Attribute;

use super::aria::Aria;
use super::audio::{Autoplay, Controls, Loop, Muted, Preload};
use super::common::Common;
use super::global::Global;
use super::input::{Height, Width};
use super::link::CrossOrigin;
use super::script::Src;
use crate::attribute::WithAttribute;
use crate::element::{Element, ElementProxy};

/// A `video` element is used for playing videos or movies, and audio files with captions.
pub fn video() -> Element<marker::Video> {
    Element::new("video")
}

pub mod marker {
    pub struct Video;

    impl<'v, V: crate::View + 'v> crate::element::IntoChild<'v, Video> for V {
        fn into_child(self) -> impl crate::View {
            self
        }
    }
}

impl<E, P> Video<(marker::Video, P)> for E where E: ElementProxy<marker::Video, P> {}
impl<E, P> Common<(marker::Video, P)> for E where E: ElementProxy<marker::Video, P> {}
impl<E, P> Global<(marker::Video, P)> for E where E: ElementProxy<marker::Video, P> {}
impl<E, P> Aria<(marker::Video, P)> for E where E: ElementProxy<marker::Video, P> {}

/// A `video` element is used for playing videos or movies, and audio files with captions.
pub trait Video<T>: WithAttribute {
    /// Address of the resource.
    fn src(self, src: impl Into<Cow<'static, str>>) -> Self {
        self.with_attribute(Src(src.into()))
    }

    /// How the element handles crossorigin requests.
    fn cross_origin(self, crossorigin: CrossOrigin) -> Self {
        self.with_attribute(crossorigin)
    }

    /// Poster frame to show prior to video playback.
    fn poster(self, poster: impl Into<Cow<'static, str>>) -> Self {
        self.with_attribute(Poster(poster.into()))
    }

    /// Hints how much buffering the media resource will likely need.
    fn preload(self, preload: Preload) -> Self {
        self.with_attribute(preload)
    }

    /// Hint that the media resource can be started automatically when the page is loaded.
    fn autoplay(self) -> Self {
        self.with_attribute(Autoplay(true))
    }

    /// Hint that the media resource can be started automatically when the page is loaded.
    fn with_autoplay(self, autoplay: bool) -> Self {
        self.with_attribute(Autoplay(autoplay))
    }

    /// Encourage the user agent to display video content within the element's playback area.
    fn plays_inline(self) -> Self {
        self.with_attribute(PlaysInline(true))
    }

    /// Encourage the user agent to display video content within the element's playback area.
    fn with_plays_inline(self, plays_inline: bool) -> Self {
        self.with_attribute(PlaysInline(plays_inline))
    }

    /// Whether to loop the media resource.
    fn loop_(self) -> Self {
        self.with_attribute(Loop(true))
    }

    /// Whether to loop the media resource.
    fn r#loop(self) -> Self {
        self.with_attribute(Loop(true))
    }

    /// Whether to loop the media resource.
    fn with_loop_(self, loop_: bool) -> Self {
        self.with_attribute(Loop(loop_))
    }

    /// Whether to mute the media resource by default.
    fn muted(self) -> Self {
        self.with_attribute(Muted(true))
    }

    /// Whether to mute the media resource by default.
    fn with_muted(self, muted: bool) -> Self {
        self.with_attribute(Muted(muted))
    }

    /// Show user agent controls.
    fn controls(self) -> Self {
        self.with_attribute(Controls(true))
    }

    /// Show user agent controls.
    fn with_controls(self, controls: bool) -> Self {
        self.with_attribute(Controls(controls))
    }

    /// Vertical dimension.
    fn height(self, height: u32) -> Self {
        self.with_attribute(Height(height))
    }

    /// Horizontal dimension.
    fn width(self, width: u32) -> Self {
        self.with_attribute(Width(width))
    }
}

/// Poster frame to show prior to video playback.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Attribute)]
pub struct Poster(pub Cow<'static, str>);

/// Encourage the user agent to display video content within the element's playback area.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Attribute)]
pub struct PlaysInline(pub bool);
