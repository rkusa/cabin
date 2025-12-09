use std::borrow::Cow;
use std::fmt;

use cabin_macros::Attribute;

use super::global::Global;
use super::option::Label;
use super::script::Src;
use crate::attribute::WithAttribute;
use crate::void_element::{VoidElement, VoidElementProxy};

/// The `track` element allows authors to specify explicit external timed text tracks for media
/// ([super::audio], [super::video]) elements. It does not represent anything on its own.
pub fn track() -> VoidElement<marker::Track> {
    VoidElement::new("track")
}

pub mod marker {
    pub struct Track;
}

impl<P> Track<marker::Track> for P where P: VoidElementProxy<marker::Track> {}
impl<P> Global<marker::Track> for P where P: VoidElementProxy<marker::Track> {}

/// The `track` element allows authors to specify explicit external timed text tracks for media
/// ([super::audio], [super::video]) elements. It does not represent anything on its own.
pub trait Track<T>: WithAttribute {
    ///  The type of text track.
    fn kind(self, kind: Kind) -> Self {
        self.with_attribute(kind)
    }

    fn kind_subtitles(self) -> Self {
        self.kind(Kind::Subtitles)
    }
    fn kind_captions(self) -> Self {
        self.kind(Kind::Captions)
    }
    fn kind_descriptions(self) -> Self {
        self.kind(Kind::Descriptions)
    }
    fn kind_chapters(self) -> Self {
        self.kind(Kind::Chapters)
    }
    fn kind_metadata(self) -> Self {
        self.kind(Kind::Metadata)
    }

    /// Address of the resource.
    fn src(self, src: impl Into<Cow<'static, str>>) -> Self {
        self.with_attribute(Src(src.into()))
    }

    /// Language of the text track.
    fn src_lang(self, src_lang: impl Into<Cow<'static, str>>) -> Self {
        self.with_attribute(SrcLang(src_lang.into()))
    }

    /// User-visible label.
    fn label(self, value: impl Into<Cow<'static, str>>) -> Self {
        self.with_attribute(Label(value.into()))
    }

    /// Enable the track if no other text track is more suitable.
    fn default(self) -> Self {
        self.with_default(true)
    }

    /// Enable the track if no other text track is more suitable.
    fn with_default(self, default: bool) -> Self {
        self.with_attribute(Default(default))
    }
}

/// Data type of an input element.
#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Attribute)]
pub enum Kind {
    /// Transcription or translation of the dialogue, suitable for when the sound is available but
    /// not understood. Overlaid on the video.
    #[default]
    Subtitles,

    /// Transcription or translation of the dialogue, sound effects, relevant musical cues, and
    /// other relevant audio information, suitable for when sound is unavailable or not clearly
    /// audible. Overlaid on the video; labeled as appropriate for the hard-of-hearing.
    Captions,

    /// Textual descriptions of the video component of the media resource, intended for audio
    /// synthesis when the visual component is obscured, unavailable, or not usable. Synthesized as
    /// audio.
    Descriptions,

    /// Track intended for use from script. Not displayed by the user agent.
    Chapters,

    /// Track intended for use from script. Not displayed by the user agent.
    Metadata,
}

impl fmt::Display for Kind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::Subtitles => "subtitles",
            Self::Captions => "captions",
            Self::Descriptions => "descriptions",
            Self::Chapters => "chapters",
            Self::Metadata => "metadata",
        })
    }
}

/// Language of a text track.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Attribute)]
pub struct SrcLang(pub Cow<'static, str>);

/// Enable a track if no other text track is more suitable.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Attribute)]
pub struct Default(pub bool);
