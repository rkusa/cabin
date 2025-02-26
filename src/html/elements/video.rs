use std::borrow::Cow;

use cabin_macros::Attribute;

use super::audio::{Autoplay, Controls, Loop, Muted, Preload};
use super::common::Common;
use super::global::Global;
use super::input::{Height, Width};
use super::link::CrossOrigin;
use super::script::Src;
use crate::View;
use crate::html::attributes::{Attributes, WithAttribute};
use crate::html::{Aria, Html};

/// A `video` element is used for playing videos or movies, and audio files with captions.
pub fn video(content: impl View) -> Html<marker::Video, (), impl View> {
    #[cfg(debug_assertions)]
    let content = content.boxed();
    Html::new("video", (), content)
}

pub mod marker {
    pub struct Video;
}

impl<A: Attributes, V: 'static> Video for Html<marker::Video, A, V> {}
impl<A: Attributes, V: 'static> Common for Html<marker::Video, A, V> {}
impl<A: Attributes, V: 'static> Global for Html<marker::Video, A, V> {}
impl<A: Attributes, V: 'static> Aria for Html<marker::Video, A, V> {}

/// A `video` element is used for playing videos or movies, and audio files with captions.
pub trait Video: WithAttribute {
    /// Address of the resource.
    fn src(self, src: impl Into<Cow<'static, str>>) -> Self::Output<Src> {
        self.with_attribute(Src(src.into()))
    }

    /// How the element handles crossorigin requests.
    fn cross_origin(self, crossorigin: CrossOrigin) -> Self::Output<CrossOrigin> {
        self.with_attribute(crossorigin)
    }

    /// Poster frame to show prior to video playback.
    fn poster(self, poster: impl Into<Cow<'static, str>>) -> Self::Output<Poster> {
        self.with_attribute(Poster(poster.into()))
    }

    /// Hints how much buffering the media resource will likely need.
    fn preload(self, preload: Preload) -> Self::Output<Preload> {
        self.with_attribute(preload)
    }

    /// Hint that the media resource can be started automatically when the page is loaded.
    fn autoplay(self) -> Self::Output<Autoplay> {
        self.with_attribute(Autoplay(true))
    }

    /// Hint that the media resource can be started automatically when the page is loaded.
    fn with_autoplay(self, autoplay: bool) -> Self::Output<Autoplay> {
        self.with_attribute(Autoplay(autoplay))
    }

    /// Encourage the user agent to display video content within the element's playback area.
    fn plays_inline(self) -> Self::Output<PlaysInline> {
        self.with_attribute(PlaysInline(true))
    }

    /// Encourage the user agent to display video content within the element's playback area.
    fn with_plays_inline(self, plays_inline: bool) -> Self::Output<PlaysInline> {
        self.with_attribute(PlaysInline(plays_inline))
    }

    /// Whether to loop the media resource.
    fn loop_(self) -> Self::Output<Loop> {
        self.with_attribute(Loop(true))
    }

    /// Whether to loop the media resource.
    fn r#loop(self) -> Self::Output<Loop> {
        self.with_attribute(Loop(true))
    }

    /// Whether to loop the media resource.
    fn with_loop_(self, loop_: bool) -> Self::Output<Loop> {
        self.with_attribute(Loop(loop_))
    }

    /// Whether to mute the media resource by default.
    fn muted(self) -> Self::Output<Muted> {
        self.with_attribute(Muted(true))
    }

    /// Whether to mute the media resource by default.
    fn with_muted(self, muted: bool) -> Self::Output<Muted> {
        self.with_attribute(Muted(muted))
    }

    /// Show user agent controls.
    fn controls(self) -> Self::Output<Controls> {
        self.with_attribute(Controls(true))
    }

    /// Show user agent controls.
    fn with_controls(self, controls: bool) -> Self::Output<Controls> {
        self.with_attribute(Controls(controls))
    }

    /// Vertical dimension.
    fn height(self, height: u32) -> Self::Output<Height> {
        self.with_attribute(Height(height))
    }

    /// Horizontal dimension.
    fn width(self, width: u32) -> Self::Output<Width> {
        self.with_attribute(Width(width))
    }
}

/// Poster frame to show prior to video playback.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Attribute)]
pub struct Poster(pub Cow<'static, str>);

/// Encourage the user agent to display video content within the element's playback area.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Attribute)]
pub struct PlaysInline(pub bool);
