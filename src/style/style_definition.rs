use std::fmt;

use smallvec::SmallVec;

use crate::style::ClassName;
use crate::style::animation::AnimationStyle;
use crate::style::modifier::StyleModifier;
use crate::style::property_display::PropertyDisplay as _;
use crate::style::units::aspect::Aspect;
use crate::style::units::box_shadow::BoxShadow;
use crate::style::units::compact::Compact;
use crate::style::units::corners::Corners;
use crate::style::units::duration::Duration;
use crate::style::units::either::Either;
use crate::style::units::float::Float;
use crate::style::units::four_sided::FourSided;
use crate::style::units::grid_lines::GridLines;
use crate::style::units::image::Image;
use crate::style::units::inlined::Inlined;
use crate::style::units::iterations::Iterations;
use crate::style::units::length::Length;
use crate::style::units::line_clamp::LineClamp;
use crate::style::units::row_column::RowColumn;
use crate::style::units::track_repeat_equally::TrackRepeatEqually;
use crate::style::units::transform::Transform;
use crate::style::units::xy::Xy;

// Large, rarely used values are boxed (`#[boxed]`), as the largest variant determines the size of
// every `Property`.
macro_rules! stored_ty {
    (boxed $ty:ty) => { Box<$ty> };
    ($ty:ty) => { $ty };
}

macro_rules! wrap_value {
    (boxed $v:expr) => {
        Box::new($v)
    };
    ($v:expr) => {
        $v
    };
}

macro_rules! unwrap_value {
    (boxed $v:expr) => {
        &mut **$v
    };
    ($v:expr) => {
        $v
    };
}

macro_rules! properties {
    ($($(#[$boxed:ident])? $name:ident: $ty:ty,)*) => {
        /// A single CSS property. The variants are declared (and thus sorted and written) in the
        /// order in which they are written to CSS, so shorthands must come before the longhands
        /// they cover.
        #[allow(non_camel_case_types)]
        #[derive(Clone, Hash, PartialEq, Eq)]
        pub(crate) enum Property {
            $($name(stored_ty!($($boxed)? $ty)),)*
        }

        #[allow(non_camel_case_types)]
        #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
        pub(crate) enum PropertyKind {
            $($name,)*
        }

        impl Property {
            #[inline]
            fn kind(&self) -> PropertyKind {
                match self {
                    $(Property::$name(_) => PropertyKind::$name,)*
                }
            }
        }

        impl MergeFrom for Property {
            fn merge_from(&mut self, other: Self) {
                match (self, other) {
                    $((Property::$name(a), Property::$name(b)) => a.merge_from(b),)*
                    (a, b) => *a = b,
                }
            }
        }

        impl StyleDefinition {
            $(
                #[inline]
                pub fn $name(&mut self) -> Slot<'_, $ty> {
                    Slot {
                        properties: &mut self.properties,
                        kind: PropertyKind::$name,
                        wrap: |v| Property::$name(wrap_value!($($boxed)? v)),
                        unwrap: |p| match p {
                            Property::$name(v) => unwrap_value!($($boxed)? v),
                            _ => unreachable!(),
                        },
                    }
                }
            )*
        }
    };
}

properties! {
    // Before `align-*` and `justify-*`, as these shorthands would override them otherwise.
    place_content: &'static str,
    place_items: &'static str,
    place_self: &'static str,
    align_items: &'static str,
    align_self: &'static str,
    animation_delay: Duration,
    animation_duration: Duration,
    animation_iterations: Iterations,
    appearance: bool,
    aspect: Aspect,
    auto_cols: Either<Length>,
    auto_rows: Either<Length>,
    background_clip: &'static str,
    background_color: &'static str,
    #[boxed]
    background_image: Image,
    basis: Length,
    border_color: Compact<FourSided<&'static str>>,
    border_inline_color: Compact<Inlined<&'static str>>,
    border_radius: Compact<Corners<Either<Length>>>,
    border_style: &'static str,
    border_width: FourSided<Length>,
    // After the `border-style` and `border-width` shorthands, so they don't override these.
    border_inline_style: &'static str,
    border_inline_width: Inlined<Length>,
    #[boxed]
    box_shadow: BoxShadow,
    box_sizing: &'static str,
    break_after: &'static str,
    break_before: &'static str,
    break_inside: &'static str,
    clip_path: &'static str,
    color: &'static str,
    container: &'static str,
    content: &'static str,
    cursor: &'static str,
    decoration: &'static str,
    display: &'static str,
    flex_direction: &'static str,
    flex_grow: bool,
    flex_shrink: bool,
    flex_wrap: &'static str,
    font_family: &'static str,
    font_size: Length,
    font_style: &'static str,
    font_weight: u16,
    gap: RowColumn<Length>,
    grid_auto_flow: &'static str,
    grid_column: GridLines,
    grid_row: GridLines,
    grid_template_columns: Either<TrackRepeatEqually>,
    grid_template_rows: Either<TrackRepeatEqually>,
    height: Length,
    inset: FourSided<Length>,
    inset_inline_end: Length,
    inset_inline_start: Length,
    justify_content: &'static str,
    justify_self: &'static str,
    letter_spacing: Length,
    line_clamp: LineClamp,
    line_height: Either<Length, Float>,
    list_style_type: &'static str,
    margin: FourSided<Length>,
    margin_inline: Inlined<Length>,
    max_height: Length,
    max_width: Length,
    min_height: Length,
    min_width: Length,
    object_fit: &'static str,
    object_position: &'static str,
    opacity: Length,
    order: i32,
    outline_color: &'static str,
    outline_offset: Length,
    outline_style: &'static str,
    outline_width: Length,
    overflow: Xy<&'static str>,
    padding: FourSided<Length>,
    pointer_events: &'static str,
    position: &'static str,
    text_align: &'static str,
    text_overflow: &'static str,
    text_transform: &'static str,
    transform: Transform,
    transition_delay: Duration,
    transition_duration: Duration,
    transition_property: &'static str,
    transition_timing_function: &'static str,
    user_select: &'static str,
    visibility: &'static str,
    white_space: &'static str,
    width: Length,
    word_break: &'static str,
    z_index: Either<i32>,
}

const INLINE_PROPERTIES: usize = 8;

/// The style of a single class. Only the properties that are set are stored, sorted by their
/// [PropertyKind], which keeps the struct small and makes hashing (for the class name) cheap.
#[derive(Default, Clone, Hash, PartialEq, Eq)]
pub struct StyleDefinition {
    pub modifier: StyleModifier,
    pub animation_from: Option<AnimationStyle>,
    pub animation_to: Option<AnimationStyle>,
    pub divide_x_reversed: bool,
    pub divide_y_reversed: bool,
    pub space_x_reversed: bool,
    pub space_y_reversed: bool,
    // Sorted by kind, at most one entry per kind, and never an entry for an unset property, as
    // `Hash` and `Eq` (and thus class names) rely on it.
    properties: SmallVec<Property, INLINE_PROPERTIES>,
}

/// Access to a single property of a [StyleDefinition].
pub struct Slot<'a, T> {
    properties: &'a mut SmallVec<Property, INLINE_PROPERTIES>,
    kind: PropertyKind,
    wrap: fn(T) -> Property,
    unwrap: fn(&mut Property) -> &mut T,
}

impl<'a, T> Slot<'a, T> {
    #[inline]
    fn position(&self) -> Result<usize, usize> {
        self.properties
            .binary_search_by(|p| p.kind().cmp(&self.kind))
    }

    /// Set the property, replacing any previous value.
    #[inline]
    pub fn set(self, value: T) {
        match self.position() {
            Ok(ix) => self.properties[ix] = (self.wrap)(value),
            Err(ix) => self.properties.insert(ix, (self.wrap)(value)),
        }
    }

    /// Get the property, inserting its default value if it isn't set yet.
    #[inline]
    pub fn get_or_insert_default(self) -> &'a mut T
    where
        T: Default,
    {
        let ix = match self.position() {
            Ok(ix) => ix,
            Err(ix) => {
                self.properties.insert(ix, (self.wrap)(T::default()));
                ix
            }
        };
        (self.unwrap)(&mut self.properties[ix])
    }

    /// Unset the property.
    #[inline]
    pub fn clear(self) {
        if let Ok(ix) = self.position() {
            self.properties.remove(ix);
        }
    }

    #[inline]
    pub fn is_set(&self) -> bool {
        self.position().is_ok()
    }
}

impl fmt::Display for StyleDefinition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let has_line_clamp = self.get(PropertyKind::line_clamp).is_some();
        for property in &self.properties {
            match property {
                Property::align_items(v) => v.fmt_property("align-items", f)?,
                Property::align_self(v) => v.fmt_property("align-self", f)?,
                Property::animation_delay(v) => v.fmt_property("animation-delay", f)?,
                Property::animation_duration(v) => v.fmt_property("animation-duration", f)?,
                Property::animation_iterations(v) => {
                    v.fmt_property("animation-iteration-count", f)?
                }
                Property::appearance(appearance) => {
                    let appearance = if *appearance { "auto" } else { "none" };
                    appearance.fmt_property("-webkit-appearance", f)?;
                    appearance.fmt_property("-moz-appearance", f)?;
                    appearance.fmt_property("appearance", f)?;
                }
                Property::aspect(v) => v.fmt_property("aspect-ratio", f)?,
                Property::auto_cols(v) => v.fmt_property("grid-auto-columns", f)?,
                Property::auto_rows(v) => v.fmt_property("grid-auto-rows", f)?,
                Property::background_clip(v) => v.fmt_property("background-clip", f)?,
                Property::background_color(v) => v.fmt_property("background-color", f)?,
                Property::background_image(v) => v.fmt_property("background-image", f)?,
                Property::basis(v) => v.fmt_property("flex-basis", f)?,
                Property::border_color(v) => v.fmt_property("border-color", f)?,
                Property::border_inline_color(v) => v.fmt_property("border-color", f)?,
                Property::border_inline_style(v) => v.fmt_property("border-inline-style", f)?,
                Property::border_inline_width(border_width) => border_width
                    .to_reversed(self.divide_x_reversed, self.divide_y_reversed)
                    .fmt_property("border-width", f)?,
                Property::border_radius(v) => v.fmt_property("border-radius", f)?,
                Property::border_style(v) => v.fmt_property("border-style", f)?,
                Property::border_width(v) => v.fmt_property("border-width", f)?,
                Property::box_shadow(v) => v.fmt_property("box-shadow", f)?,
                Property::box_sizing(v) => v.fmt_property("box-sizing", f)?,
                Property::break_after(v) => v.fmt_property("break-after", f)?,
                Property::break_before(v) => v.fmt_property("break-before", f)?,
                Property::break_inside(v) => v.fmt_property("break-inside", f)?,
                Property::clip_path(v) => v.fmt_property("clip-path", f)?,
                Property::color(v) => v.fmt_property("color", f)?,
                Property::container(v) => v.fmt_property("container-type", f)?,
                Property::content(content) => {
                    if content.is_empty() {
                        r#""""#.fmt_property("content", f)?;
                    } else {
                        content.fmt_property("content", f)?;
                    }
                }
                Property::cursor(v) => v.fmt_property("cursor", f)?,
                Property::decoration(v) => v.fmt_property("text-decoration-line", f)?,
                Property::display(display) => {
                    if !has_line_clamp {
                        display.fmt_property("display", f)?;
                    }
                }
                Property::flex_direction(v) => v.fmt_property("flex-direction", f)?,
                Property::flex_grow(v) => v.fmt_property("flex-grow", f)?,
                Property::flex_shrink(v) => v.fmt_property("flex-shrink", f)?,
                Property::flex_wrap(v) => v.fmt_property("flex-wrap", f)?,
                Property::font_family(v) => v.fmt_property("font-family", f)?,
                Property::font_size(v) => v.fmt_property("font-size", f)?,
                Property::font_style(v) => v.fmt_property("font-style", f)?,
                Property::font_weight(v) => v.fmt_property("font-weight", f)?,
                Property::gap(v) => v.fmt_property("gap", f)?,
                Property::grid_auto_flow(v) => v.fmt_property("grid-auto-flow", f)?,
                Property::grid_column(v) => v.fmt_property("grid-column", f)?,
                Property::grid_row(v) => v.fmt_property("grid-row", f)?,
                Property::grid_template_columns(v) => v.fmt_property("grid-template-columns", f)?,
                Property::grid_template_rows(v) => v.fmt_property("grid-template-rows", f)?,
                Property::height(v) => v.fmt_property("height", f)?,
                Property::inset(v) => v.fmt_property("inset", f)?,
                Property::inset_inline_end(v) => v.fmt_property("inset-inline-end", f)?,
                Property::inset_inline_start(v) => v.fmt_property("inset-inline-start", f)?,
                Property::justify_content(v) => v.fmt_property("justify-content", f)?,
                Property::justify_self(v) => v.fmt_property("justify-self", f)?,
                Property::letter_spacing(v) => v.fmt_property("letter-spacing", f)?,
                Property::line_clamp(line_clamp) => fmt::Display::fmt(line_clamp, f)?,
                Property::line_height(v) => v.fmt_property("line-height", f)?,
                Property::list_style_type(v) => v.fmt_property("list-style-type", f)?,
                Property::margin(v) => v.fmt_property("margin", f)?,
                Property::margin_inline(margin_inline) => margin_inline
                    .to_reversed(self.space_x_reversed, self.space_y_reversed)
                    .fmt_property("margin", f)?,
                Property::max_height(v) => v.fmt_property("max-height", f)?,
                Property::max_width(v) => v.fmt_property("max-width", f)?,
                Property::min_height(v) => v.fmt_property("min-height", f)?,
                Property::min_width(v) => v.fmt_property("min-width", f)?,
                Property::object_fit(v) => v.fmt_property("object-fit", f)?,
                Property::object_position(v) => v.fmt_property("object-position", f)?,
                Property::opacity(v) => v.fmt_property("opacity", f)?,
                Property::order(v) => v.fmt_property("order", f)?,
                Property::outline_color(v) => v.fmt_property("outline-color", f)?,
                Property::outline_offset(v) => v.fmt_property("outline-offset", f)?,
                Property::outline_style(v) => v.fmt_property("outline-style", f)?,
                Property::outline_width(v) => v.fmt_property("outline-width", f)?,
                Property::overflow(overflow) => {
                    if !has_line_clamp {
                        overflow.fmt_property("overflow", f)?;
                    }
                }
                Property::padding(v) => v.fmt_property("padding", f)?,
                Property::place_content(v) => v.fmt_property("place-content", f)?,
                Property::place_items(v) => v.fmt_property("place-items", f)?,
                Property::place_self(v) => v.fmt_property("place-self", f)?,
                Property::pointer_events(v) => v.fmt_property("pointer-events", f)?,
                Property::position(v) => v.fmt_property("position", f)?,
                Property::text_align(v) => v.fmt_property("text-align", f)?,
                Property::text_overflow(v) => v.fmt_property("text-overflow", f)?,
                Property::text_transform(v) => v.fmt_property("text-transform", f)?,
                Property::transform(v) => v.fmt_property("transform", f)?,
                Property::transition_delay(v) => v.fmt_property("transition-delay", f)?,
                Property::transition_duration(v) => v.fmt_property("transition-duration", f)?,
                Property::transition_property(v) => v.fmt_property("transition-property", f)?,
                Property::transition_timing_function(v) => {
                    v.fmt_property("transition-timing-function", f)?
                }
                Property::user_select(user_select) => {
                    user_select.fmt_property("-webkit-user-select", f)?;
                    user_select.fmt_property("user-select", f)?;
                }
                Property::visibility(v) => v.fmt_property("visibility", f)?,
                Property::white_space(v) => v.fmt_property("white-space", f)?,
                Property::width(v) => v.fmt_property("width", f)?,
                Property::word_break(v) => v.fmt_property("word-break", f)?,
                Property::z_index(v) => v.fmt_property("z-index", f)?,
            }
        }
        if let Some(Property::word_break(word_break)) = self.get(PropertyKind::word_break)
            && *word_break == "normal"
        {
            word_break.fmt_property("overflow-wrap", f)?;
        }

        Ok(())
    }
}

impl StyleDefinition {
    /// Whether the style results in any CSS. The `*_reversed` flags don't on their own.
    pub(crate) fn is_empty(&self) -> bool {
        self.properties.is_empty() && self.animation_from.is_none() && self.animation_to.is_none()
    }

    pub(crate) fn new(modifier: StyleModifier) -> Self {
        Self {
            modifier,
            ..Default::default()
        }
    }

    fn index_of(&self, kind: PropertyKind) -> Result<usize, usize> {
        self.properties.binary_search_by(|p| p.kind().cmp(&kind))
    }

    fn get(&self, kind: PropertyKind) -> Option<&Property> {
        self.index_of(kind).ok().map(|ix| &self.properties[ix])
    }

    pub fn class_name(&self) -> ClassName {
        ClassName::new(self)
    }

    pub fn inherit(&mut self, parent: &Self) {
        let Ok(ix) = self.index_of(PropertyKind::background_image) else {
            return;
        };
        let Some(Property::background_image(parent)) = parent.get(PropertyKind::background_image)
        else {
            return;
        };
        if let Property::background_image(background_image) = &mut self.properties[ix] {
            let own = std::mem::replace(background_image, parent.clone());
            background_image.merge_from(own);
        }
    }

    pub fn merge_from(&mut self, other: Self) {
        let StyleDefinition {
            modifier: _,
            animation_from,
            animation_to,
            divide_x_reversed,
            divide_y_reversed,
            space_x_reversed,
            space_y_reversed,
            properties,
        } = other;

        self.animation_from.merge_from(animation_from);
        self.animation_to.merge_from(animation_to);
        self.divide_x_reversed.merge_from(divide_x_reversed);
        self.divide_y_reversed.merge_from(divide_y_reversed);
        self.space_x_reversed.merge_from(space_x_reversed);
        self.space_y_reversed.merge_from(space_y_reversed);
        for property in properties {
            match self.index_of(property.kind()) {
                Ok(ix) => self.properties[ix].merge_from(property),
                Err(ix) => self.properties.insert(ix, property),
            }
        }
    }

    pub(crate) fn write_to(&self, out: &mut dyn fmt::Write) {
        let class_name = self.class_name();

        let StyleModifier {
            active,
            disabled,
            enabled,
            focus,
            focus_visible,
            focus_within,
            hover,
            visited,
            after,
            before,
            group_hover,
            all_children,
            all_but_last_children,
            max_width,
            min_width,
            max_container_width,
            min_container_width,
            print,
            dark,
            other_pseudo_element,
        } = self.modifier;

        // @keyframes
        let has_animation = self.animation_from.is_some() || self.animation_to.is_some();
        if has_animation {
            write!(out, "@keyframes {class_name} {{ from {{").unwrap();
            if let Some(animation_from) = &self.animation_from {
                write!(out, "{animation_from}").unwrap();
            }
            writeln!(out, "}} to {{").unwrap();
            if let Some(animation_to) = &self.animation_to {
                write!(out, "{animation_to}").unwrap();
            }
            writeln!(out, "}} }}").unwrap();
        }

        // @media {}
        let has_media_query = print || dark || max_width.is_some() || min_width.is_some();
        if has_media_query {
            write!(out, "@media ").unwrap();
            let mut write_and = false;
            if print {
                write!(out, "print").unwrap();
                write_and = true;
            }
            if dark {
                if write_and {
                    write!(out, " and ").unwrap();
                }
                write!(out, "(prefers-color-scheme: dark)").unwrap();
                write_and = true;
            }
            if let Some(min_width) = min_width {
                if write_and {
                    write!(out, " and ").unwrap();
                }
                write!(out, "(min-width: {min_width}px)").unwrap();
                write_and = true;
            }
            if let Some(max_width) = max_width {
                if write_and {
                    write!(out, " and ").unwrap();
                }
                write!(out, "(max-width: {max_width}px)").unwrap();
                // write_and = true;
            }
            write!(out, "{{ ").unwrap();
        }

        // @container {}
        let has_container_query = max_container_width.is_some() || min_container_width.is_some();
        if has_container_query {
            write!(out, "@container ").unwrap();
            let mut write_and = false;
            if let Some(min_width) = min_container_width {
                write!(out, "(min-width: {min_width}px)").unwrap();
                write_and = true;
            }
            if let Some(max_width) = max_container_width {
                if write_and {
                    write!(out, " and ").unwrap();
                }
                write!(out, "(max-width: {max_width}px)").unwrap();
                // write_and = true;
            }
            write!(out, "{{ ").unwrap();
        }

        if group_hover {
            write!(out, ".group:hover ").unwrap();
        }
        if all_children || all_but_last_children {
            write!(out, ":where(").unwrap();
        }

        write!(out, ".{class_name}").unwrap();

        if active {
            write!(out, ":active").unwrap();
        }
        if disabled {
            write!(out, ":disabled").unwrap();
        }
        if enabled {
            write!(out, ":enabled").unwrap();
        }
        if focus {
            write!(out, ":focus").unwrap();
        }
        if focus_visible {
            write!(out, ":focus-visible").unwrap();
        }
        if focus_within {
            write!(out, ":focus-within").unwrap();
        }
        if hover {
            write!(out, ":hover").unwrap();
        }
        if visited {
            write!(out, ":visited").unwrap();
        }
        if dark {
            write!(out, ":not(.force-light *)").unwrap();
        }
        if after {
            write!(out, "::after").unwrap();
        }
        if before {
            write!(out, "::before").unwrap();
        }
        if let Some(pseudo) = other_pseudo_element {
            write!(out, "::{pseudo}").unwrap();
        }

        if all_children {
            write!(out, " > *)").unwrap();
        } else if all_but_last_children {
            write!(out, " > :not(:last-child))").unwrap();
        }

        writeln!(out, " {{").unwrap();
        if has_animation {
            write!(out, "animation: 250ms ease-in-out 1 forwards {class_name};").unwrap();
        }
        write!(out, "{self}").unwrap();
        write!(out, "}}").unwrap();

        if has_container_query {
            write!(out, " }}").unwrap();
        }
        if has_media_query {
            write!(out, " }}").unwrap();
        }

        writeln!(out).unwrap();
    }
}

pub trait MergeFrom {
    fn merge_from(&mut self, other: Self);
}

impl MergeFrom for StyleDefinition {
    fn merge_from(&mut self, other: Self) {
        StyleDefinition::merge_from(self, other);
    }
}

impl<T: MergeFrom> MergeFrom for Box<T> {
    fn merge_from(&mut self, other: Self) {
        (**self).merge_from(*other);
    }
}

impl<T: MergeFrom> MergeFrom for Option<T> {
    fn merge_from(&mut self, other: Self) {
        let Some(other) = other else {
            return;
        };

        if let Some(s) = self {
            s.merge_from(other);
        } else {
            *self = Some(other);
        }
    }
}

impl MergeFrom for &'static str {
    fn merge_from(&mut self, other: Self) {
        *self = other;
    }
}

impl MergeFrom for u16 {
    fn merge_from(&mut self, other: Self) {
        *self = other;
    }
}

impl MergeFrom for i32 {
    fn merge_from(&mut self, other: Self) {
        *self = other;
    }
}

impl MergeFrom for bool {
    fn merge_from(&mut self, other: Self) {
        *self = *self || other;
    }
}
