use std::borrow::Cow;
use std::fmt;

use cabin_macros::Attribute;

use super::common::Common;
use super::global::Global;
use super::link::CrossOrigin;
use super::script::Src;
use crate::View;
use crate::html::attributes::{Attributes, WithAttribute};
use crate::html::{Aria, Html};

/// An `audio` element represents a sound or audio stream.
pub fn audio(content: impl View) -> Html<marker::Audio, ()> {
    #[cfg(debug_assertions)]
    let content = content.boxed();
    Html::new("audio", (), content)
}

mod macros {
    #[macro_export]
    macro_rules! audio {
        ($($x:tt)*) => {
            $crate::html::elements::audio::audio($crate::view![$($x)*])
        }
    }

    pub use audio;
}

pub use macros::audio;

pub mod marker {
    pub struct Audio;
}

impl<A: Attributes> Audio for Html<marker::Audio, A> {}
impl<A: Attributes> Common for Html<marker::Audio, A> {}
impl<A: Attributes> Global for Html<marker::Audio, A> {}
impl<A: Attributes> Aria for Html<marker::Audio, A> {}

/// An `audio` element represents a sound or audio stream.
pub trait Audio: WithAttribute {
    /// Address of the resource.
    fn src(self, src: impl Into<Cow<'static, str>>) -> Self::Output<Src> {
        self.with_attribute(Src(src.into()))
    }

    /// How the element handles crossorigin requests.
    fn cross_origin(self, crossorigin: CrossOrigin) -> Self::Output<CrossOrigin> {
        self.with_attribute(crossorigin)
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
}

/// Hints how much buffering the media resource will likely need.
#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Attribute)]
pub enum Preload {
    /// Hints to the user agent that either the author does not expect the user to need the media
    /// resource, or that the server wants to minimize unnecessary traffic.
    None,

    /// Hints to the user agent that the author does not expect the user to need the media
    /// resource, but that fetching the resource metadata (dimensions, track list, duration,
    /// etc.), and maybe even the first few frames, is reasonable.
    Metadata,

    /// Hints to the user agent that the user agent can put the user's needs first without risk to
    /// the server, up to and including optimistically downloading the entire resource.
    #[default]
    Auto,
}

impl fmt::Display for Preload {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Preload::None => "none",
            Preload::Metadata => "metadata",
            Preload::Auto => "auto",
        })
    }
}

/// Hint that the media resource can be started automatically when the page is loaded.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Attribute)]
pub struct Autoplay(pub bool);

/// Whether to loop the media resource.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Attribute)]
pub struct Loop(pub bool);

/// Whether to mute the media resource by default.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Attribute)]
pub struct Muted(pub bool);

/// Show user agent controls.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Attribute)]
pub struct Controls(pub bool);
